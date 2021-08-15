extern crate nalgebra as na;
use na::{Rotation2, Rotation3, Vector2, Vector3};

use uom::si::{
    acceleration::meter_per_second_squared, angle::radian, f64::*, force::newton, length::meter,
    mass::kilogram,
};

use crate::simulation::UpdateContext;

// RigidBodyOnHinge represent any physical object able to rotate on a hinge axis.
// It can be a gear, elevator, cargo door...... Only one rotation degree of freedom is handled.
// An actuator of multiple actuators can to apply forces to its control arm
#[derive(PartialEq, Clone, Copy)]
pub struct RigidBodyOnHingeAxis {
    throw: f64,
    min_angle: f64,
    max_angle: f64,

    size: Vector3<f64>,

    center_of_gravity_offset: Vector2<f64>,
    center_of_gravity_actual: Vector2<f64>,

    control_arm: Vector2<f64>,
    control_arm_actual: Vector2<f64>,

    anchor_point: Vector2<f64>,

    position: f64,
    speed: f64,
    acceleration: f64,
    sum_of_torques: f64,

    position_normalized: f64,
    position_normalized_prev: f64,

    mass: Mass,
    inertia_at_hinge: f64,

    natural_damping: f64,
    max_speed: f64,

    lock_position_request: f64,
    is_lock_requested: bool,
    is_locked: bool,
}
impl RigidBodyOnHingeAxis {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        mass: Mass,
        size: Vector3<f64>,
        center_of_gravity_offset: Vector2<f64>,
        control_arm: Vector2<f64>,
        anchor_point: Vector2<f64>,
        min_angle: Angle,
        throw: Angle,
        natural_damping: f64,
        locked: bool,
    ) -> Self {
        let inertia_at_cog =
            (1. / 12.) * mass.get::<kilogram>() * size[0] * size[0] + size[1] * size[1];

        // Parallel axis theorem to get inertia at hinge axis from inertia at CoG
        let inertia_at_hinge =
            inertia_at_cog + mass.get::<kilogram>() * center_of_gravity_offset.norm_squared();

        let mut new_body = RigidBodyOnHingeAxis {
            throw: throw.get::<radian>(),
            min_angle: min_angle.get::<radian>(),
            max_angle: min_angle.get::<radian>() + throw.get::<radian>(),
            size,
            center_of_gravity_offset,
            center_of_gravity_actual: center_of_gravity_offset,
            control_arm,
            control_arm_actual: control_arm,
            anchor_point,
            position: min_angle.get::<radian>(),
            speed: 0.,
            acceleration: 0.,
            sum_of_torques: 0.,
            position_normalized: 0.,
            position_normalized_prev: 0.,
            mass,
            inertia_at_hinge,
            natural_damping,
            max_speed: 4.0,
            lock_position_request: min_angle.get::<radian>(),
            is_lock_requested: locked,
            is_locked: locked,
        };

        new_body.update_all_rotations();
        new_body.update_position_normalized();
        new_body
    }

    pub fn apply_control_arm_force(&mut self, force: Force) {
        // Computing the normalized vector on which force is applied. This is the vector from anchor point of actuator to where
        // it's connected to the rigid body
        let force_support_vector_2d = self.anchor_point - self.control_arm_actual;
        let force_support_vector_2d_normalized =
            force_support_vector_2d / force_support_vector_2d.norm();

        // Adding a dummy 0 component to get 3D coordinates
        let force_support_vector_3d_normalized = Vector3::new(
            force_support_vector_2d_normalized[0],
            force_support_vector_2d_normalized[1],
            0.,
        );

        let control_arm_3d =
            Vector3::new(self.control_arm_actual[0], self.control_arm_actual[1], 0.);

        // Final moment if magnitude of the force applied on the force support vector, cross product with
        // control arm position relative to hinge.
        let torque =
            (force.get::<newton>() * force_support_vector_3d_normalized).cross(&control_arm_3d);

        let torque_value = torque[2];

        self.sum_of_torques += torque_value;
    }

    pub fn linear_extension_to_anchor(&self) -> Length {
        Length::new::<meter>((self.anchor_point - self.control_arm_actual).norm())
    }

    pub fn min_linear_distance_to_anchor(&self) -> Length {
        let rotation_min = Rotation2::new(self.min_angle);
        let control_arm_min = rotation_min * self.control_arm;

        Length::new::<meter>((self.anchor_point - control_arm_min).norm())
    }

    pub fn max_linear_distance_to_anchor(&self) -> Length {
        let rotation_max = Rotation2::new(self.max_angle);
        let control_arm_max = rotation_max * self.control_arm;

        Length::new::<meter>((self.anchor_point - control_arm_max).norm())
    }

    fn lock_requested_position_in_absolute_reference(&self) -> f64 {
        self.lock_position_request * self.throw + self.min_angle
    }

    pub fn position_normalized(&self) -> f64 {
        self.position_normalized
    }

    fn update_position_normalized(&mut self) {
        self.position_normalized_prev = self.position_normalized;

        self.position_normalized = (self.position - self.min_angle) / self.throw;
    }

    fn update_all_rotations(&mut self) {
        let rotation_transform = Rotation2::new(self.position);
        self.control_arm_actual = rotation_transform * self.control_arm;
        self.center_of_gravity_actual = rotation_transform * self.center_of_gravity_offset;
    }

    fn gravity_force(&self, context: &UpdateContext) -> f64 {
        let plane_acceleration_plane_reference = Vector3::new(
            context.lat_accel().get::<meter_per_second_squared>(),
            context.vert_accel().get::<meter_per_second_squared>(),
            context.long_accel().get::<meter_per_second_squared>(),
        );

        let pitch_rotation =
            Rotation3::from_axis_angle(&Vector3::x_axis(), context.pitch().get::<radian>());

        let bank_rotation =
            Rotation3::from_axis_angle(&Vector3::z_axis(), -context.bank().get::<radian>());

        let gravity_acceleration_world_reference = Vector3::new(0., -9.8, 0.);

        // Total acceleration in plane reference is the gravity in world reference rotated to plane reference. To this we substract
        // the local plane reference to get final local acceleration (if plane falling at 1G final local accel is 1G of gravity - 1G local accel = 0G)
        let total_acceleration_plane_reference = (pitch_rotation
            * (bank_rotation * gravity_acceleration_world_reference))
            - plane_acceleration_plane_reference;

        // We add a 0 component to make the 2D CG position a 3D vector so we can compute a cross product easily
        let center_of_gravity_3d = Vector3::new(
            self.center_of_gravity_actual[0],
            self.center_of_gravity_actual[1],
            0.,
        );

        // Force = m * G
        let resultant_force_plane_reference =
            total_acceleration_plane_reference * self.mass.get::<kilogram>();

        // The Moment generated by acceleration force is the CoG offset from hinge position cross product with the acceleration force
        let gravity_moment_vector = center_of_gravity_3d.cross(&resultant_force_plane_reference);

        // We work with only one degree of freedom so final result holds in the hinge rotation component only
        gravity_moment_vector[2]
    }

    fn natural_damping(&self) -> f64 {
        -self.speed * self.natural_damping
    }

    pub fn update(&mut self, context: &UpdateContext) {
        if !self.is_locked {
            self.sum_of_torques += self.natural_damping() + self.gravity_force(context);
            self.acceleration = self.sum_of_torques / self.inertia_at_hinge;

            self.speed += self.acceleration * context.delta_as_secs_f64();

            self.position += self.speed * context.delta_as_secs_f64();

            if self.is_lock_requested {
                if self.position_normalized >= self.lock_position_request
                    && self.position_normalized_prev <= self.lock_position_request
                    || self.position_normalized <= self.lock_position_request
                        && self.position_normalized_prev >= self.lock_position_request
                {
                    self.is_locked = true;
                    self.position = self.lock_requested_position_in_absolute_reference();
                    self.speed = 0.;
                }
            } else if self.position >= self.max_angle {
                self.position = self.max_angle;
                self.speed = -self.speed * 0.3;
            } else if self.position <= self.min_angle {
                self.position = self.min_angle;
                self.speed = -self.speed * 0.3;
            }
            self.update_position_normalized();
            self.update_all_rotations();
        }

        self.sum_of_torques = 0.;
    }

    pub fn unlock(&mut self) {
        self.is_locked = false;
        self.is_lock_requested = false;
    }

    pub fn lock_at_position_normalized(&mut self, position_normalized: f64) {
        self.is_lock_requested = true;
        self.lock_position_request = position_normalized;
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;
    use uom::si::{
        acceleration::meter_per_second_squared, angle::degree, length::foot,
        thermodynamic_temperature::degree_celsius, velocity::knot,
    };

    #[test]
    fn body_gravity_movement() {
        let mut rigid_body = cargo_door_body(false);

        let dt = 0.05;

        let mut time = 0.;
        for _ in 0..100 {
            rigid_body.update(&context(
                Duration::from_secs_f64(dt),
                Angle::new::<degree>(0.),
                Angle::new::<degree>(-45.),
            ));
            time += dt;
            println!("Pos {} t={}", rigid_body.position, time);
        }
    }

    #[test]
    fn not_locked_at_init_will_move() {
        let mut rigid_body = cargo_door_body(false);
        let init_pos = rigid_body.position;

        let dt = 0.05;

        let mut time = 0.;
        for _ in 0..100 {
            rigid_body.update(&context(
                Duration::from_secs_f64(dt),
                Angle::new::<degree>(0.),
                Angle::new::<degree>(-45.),
            ));
            time += dt;
            println!("Pos {} t={}", rigid_body.position, time);
            assert!((rigid_body.position - init_pos).abs() > f64::EPSILON);
        }
    }

    #[test]
    fn locked_at_init_wont_move() {
        let mut rigid_body = cargo_door_body(true);

        let dt = 0.05;

        let init_pos = rigid_body.position;

        let mut time = 0.;
        for _ in 0..100 {
            rigid_body.update(&context(
                Duration::from_secs_f64(dt),
                Angle::new::<degree>(0.),
                Angle::new::<degree>(-45.),
            ));
            time += dt;
            println!("Pos {} t={}", rigid_body.position, time);
            assert!((rigid_body.position - init_pos).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn start_moving_once_unlocked() {
        let mut rigid_body = cargo_door_body(true);

        let dt = 0.05;

        let init_pos = rigid_body.position;

        let mut time = 0.;
        for _ in 0..100 {
            rigid_body.update(&context(
                Duration::from_secs_f64(dt),
                Angle::new::<degree>(0.),
                Angle::new::<degree>(-45.),
            ));
            time += dt;

            if time < 1. {
                assert!((rigid_body.position - init_pos).abs() < f64::EPSILON);
            }

            if time >= 1. && time < 1. + dt {
                rigid_body.unlock();
                println!("UNLOCK t={}", time);
            }

            if time > 1. + dt {
                assert!((rigid_body.position - init_pos).abs() > f64::EPSILON);
            }

            println!("Pos {} t={}", rigid_body.position_normalized(), time);
        }
    }

    #[test]
    fn locks_at_required_position() {
        let mut rigid_body = cargo_door_body(false);

        let dt = 0.05;

        let mut time = 0.;

        rigid_body.lock_at_position_normalized(0.5);

        assert!(rigid_body.is_lock_requested);

        assert!(!rigid_body.is_locked);

        for _ in 0..100 {
            rigid_body.update(&context(
                Duration::from_secs_f64(dt),
                Angle::new::<degree>(0.),
                Angle::new::<degree>(-45.),
            ));
            time += dt;

            println!("Pos {} t={}", rigid_body.position_normalized(), time);
        }

        assert!(rigid_body.is_locked);
        assert!((rigid_body.position_normalized() - 0.5).abs() < f64::EPSILON);
    }

    fn context(delta_time: Duration, pitch: Angle, bank: Angle) -> UpdateContext {
        UpdateContext::new(
            delta_time,
            Velocity::new::<knot>(250.),
            Length::new::<foot>(5000.),
            ThermodynamicTemperature::new::<degree_celsius>(25.0),
            true,
            Acceleration::new::<meter_per_second_squared>(0.),
            Acceleration::new::<meter_per_second_squared>(0.),
            Acceleration::new::<meter_per_second_squared>(0.),
            pitch,
            bank,
        )
    }

    fn cargo_door_body(is_locked: bool) -> RigidBodyOnHingeAxis {
        let size = Vector3::new(100. / 1000., 1855. / 1000., 2025. / 1000.);
        let cg_offset = Vector2::new(0., -size[1] / 2.);

        let control_arm = Vector2::new(-0.1597, -0.1614);
        let anchor = Vector2::new(-0.759, -0.086);

        RigidBodyOnHingeAxis::new(
            Mass::new::<kilogram>(130.),
            size,
            cg_offset,
            control_arm,
            anchor,
            Angle::new::<degree>(-23.),
            Angle::new::<degree>(136.),
            100.,
            is_locked,
        )
    }
}
