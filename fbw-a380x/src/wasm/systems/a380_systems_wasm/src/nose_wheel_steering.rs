use std::error::Error;
use systems::shared::to_bool;
use systems_wasm::aspects::{
    EventToVariableMapping, ExecuteOn, MsfsAspectBuilder, VariableToEventMapping,
    VariableToEventWriteOn,
};
use systems_wasm::Variable;

pub(super) fn nose_wheel_steering(builder: &mut MsfsAspectBuilder) -> Result<(), Box<dyn Error>> {
    // The rudder pedals should start in a centered position.
    builder.init_variable(Variable::aspect("RAW_RUDDER_PEDAL_POSITION"), 0.5);

    builder.map(
        ExecuteOn::PreTick,
        Variable::named("RUDDER_PEDAL_POSITION"),
        // Convert rudder pedal position to [-1;1], -1 is left
        |value| ((value + 100.) / 200.) * 2. - 1.,
        Variable::aspect("RAW_RUDDER_PEDAL_POSITION"),
    );

    builder.map_many(
        ExecuteOn::PostTick,
        vec![
            Variable::named("REALISTIC_TILLER_ENABLED"),
            Variable::aspect("RAW_RUDDER_PEDAL_POSITION"),
        ],
        |values| {
            let realistic_tiller_enabled = to_bool(values[0]);
            let rudder_pedal_position = values[1];
            if realistic_tiller_enabled {
                rudder_pedal_position
            } else {
                0.
            }
        },
        Variable::aspect("RUDDER_PEDAL_POSITION_RATIO"),
    );

    // The tiller handle should start in a centered position.
    builder.init_variable(Variable::aspect("RAW_TILLER_HANDLE_POSITION"), 0.5);

    // This axis is kept for legacy reasons and was used before the steering axis was available
    builder.event_to_variable(
        "AXIS_MIXTURE4_SET",
        EventToVariableMapping::EventData32kPosition,
        Variable::aspect("RAW_TILLER_HANDLE_POSITION"),
        |options| options.mask(),
    )?;

    builder.event_to_variable(
        "AXIS_STEERING_SET",
        EventToVariableMapping::EventData32kPositionInverted,
        Variable::aspect("RAW_TILLER_HANDLE_POSITION"),
        |options| options.mask(),
    )?;

    const TILLER_KEYBOARD_INCREMENTS: f64 = 0.05;
    builder.event_to_variable(
        "STEERING_INC",
        EventToVariableMapping::CurrentValueToValue(|current_value| {
            recenter_when_close_to_center(
                (current_value + TILLER_KEYBOARD_INCREMENTS).min(1.),
                TILLER_KEYBOARD_INCREMENTS,
            )
        }),
        Variable::aspect("RAW_TILLER_HANDLE_POSITION"),
        |options| options.mask(),
    )?;
    builder.event_to_variable(
        "STEERING_DEC",
        EventToVariableMapping::CurrentValueToValue(|current_value| {
            recenter_when_close_to_center(
                (current_value - TILLER_KEYBOARD_INCREMENTS).max(0.),
                TILLER_KEYBOARD_INCREMENTS,
            )
        }),
        Variable::aspect("RAW_TILLER_HANDLE_POSITION"),
        |options| options.mask(),
    )?;

    // Lacking a better event to bind to, we've picked the toggle water rudder event for
    // disconnecting the rudder pedals via the PEDALS DISC button on the tiller.
    builder.event_to_variable(
        "TOGGLE_WATER_RUDDER",
        EventToVariableMapping::Value(1.),
        Variable::aspect("TILLER_PEDAL_DISCONNECT"),
        |options| options.mask().afterwards_reset_to(0.),
    )?;

    builder.map_many(
        ExecuteOn::PostTick,
        vec![
            Variable::named("REALISTIC_TILLER_ENABLED"),
            Variable::aspect("RAW_RUDDER_PEDAL_POSITION"),
            Variable::aspect("RAW_TILLER_HANDLE_POSITION"),
            Variable::aspect("TILLER_PEDAL_DISCONNECT"),
        ],
        |values| {
            let realistic_tiller_enabled = to_bool(values[0]);
            let rudder_pedal_position = values[1];
            let tiller_handle_position = values[2];
            let tiller_pedal_disconnect = to_bool(values[3]);

            if realistic_tiller_enabled {
                // Convert tiller handle position to [-1;1], -1 is left
                tiller_handle_position * 2. - 1.
            } else if !tiller_pedal_disconnect {
                rudder_pedal_position
            } else {
                0.
            }
        },
        Variable::named("TILLER_HANDLE_POSITION"),
    );

    builder.map(
        ExecuteOn::PostTick,
        Variable::aspect("NOSE_WHEEL_POSITION_RATIO"),
        steering_animation_to_msfs_from_steering_angle,
        Variable::named("NOSE_WHEEL_POSITION"),
    );

    builder.map_many(
        ExecuteOn::PostTick,
        vec![
            Variable::aspect("NOSE_WHEEL_POSITION_RATIO"),
            Variable::aircraft("RUDDER POSITION", "Position", 0),
        ],
        |values| {
            let nose_wheel_position = values[0];
            let rudder_position = (values[1] + 1.) / 2.;

            steering_demand_to_msfs_from_steering_angle(nose_wheel_position, rudder_position)
        },
        Variable::aspect("STEERING_ANGLE_COMMAND"),
    );

    builder.map(
        ExecuteOn::PostTick,
        Variable::aspect("NOSE_WHEEL_POSITION_RATIO"),
        steering_max_demand_to_msfs_from_steering_angle,
        Variable::aspect("STEERING_ANGLE_MAX_COMMAND"),
    );

    builder.variable_to_event(
        Variable::aspect("STEERING_ANGLE_COMMAND"),
        VariableToEventMapping::EventData32kPosition,
        VariableToEventWriteOn::EveryTick,
        "STEERING_SET",
    )?;

    builder.variable_to_event(
        Variable::aspect("STEERING_ANGLE_MAX_COMMAND"),
        VariableToEventMapping::EventData32kPosition,
        VariableToEventWriteOn::EveryTick,
        "NOSE_WHEEL_STEERING_LIMIT_SET",
    )?;

    // Adds rotational speed to nose wheel based on steering angle
    const STEERING_RATIO_TO_WHEEL_ANGLE_GAIN: f64 = 80.;
    builder.map_many(
        ExecuteOn::PostTick,
        vec![
            Variable::named("NOSE_WHEEL_POSITION"),
            Variable::aircraft("CENTER WHEEL ROTATION ANGLE", "degree", 0),
        ],
        |values| {
            normalise_angle(values[1] + (values[0] - 0.5) * STEERING_RATIO_TO_WHEEL_ANGLE_GAIN)
        },
        Variable::named("NOSE_WHEEL_LEFT_ANIM_ANGLE"),
    );
    builder.map_many(
        ExecuteOn::PostTick,
        vec![
            Variable::named("NOSE_WHEEL_POSITION"),
            Variable::aircraft("CENTER WHEEL ROTATION ANGLE", "degree", 0),
        ],
        |values| {
            normalise_angle(values[1] - (values[0] - 0.5) * STEERING_RATIO_TO_WHEEL_ANGLE_GAIN)
        },
        Variable::named("NOSE_WHEEL_RIGHT_ANIM_ANGLE"),
    );

    // Converting nose angle to rear steering angle: there's a 20° deadband before activating rear steering
    builder.map(
        ExecuteOn::PostTick,
        Variable::named("NOSE_WHEEL_POSITION"),
        |value| {
            println!(
                "NOSE STEER VALUE {:.2} output {:.2}",
                value,
                (4. * (value - 0.5).powi(3) + 0.5).clamp(0., 1.)
            );
            (4. * (value - 0.5).powi(3) + 0.5).clamp(0., 1.)
        },
        Variable::named("REAR_WHEEL_STEERING_POSITION"),
    );

    // Rear steering wheel animations
    // Adds rotational speed to bws wheel based on steering angle
    const REAR_STEERING_RATIO_TO_WHEEL_ANGLE_GAIN: f64 = 40.;
    builder.map_many(
        ExecuteOn::PostTick,
        vec![
            Variable::named("REAR_WHEEL_STEERING_POSITION"),
            Variable::aircraft("LEFT WHEEL ROTATION ANGLE", "degree", 0),
        ],
        |values| {
            normalise_angle(values[1] + (values[0] - 0.5) * REAR_STEERING_RATIO_TO_WHEEL_ANGLE_GAIN)
        },
        Variable::named("LEFT_REAR_STEERING_WHEEL_RIGHT_ANIM_ANGLE"),
    );
    builder.map_many(
        ExecuteOn::PostTick,
        vec![
            Variable::named("REAR_WHEEL_STEERING_POSITION"),
            Variable::aircraft("LEFT WHEEL ROTATION ANGLE", "degree", 0),
        ],
        |values| {
            normalise_angle(values[1] - (values[0] - 0.5) * REAR_STEERING_RATIO_TO_WHEEL_ANGLE_GAIN)
        },
        Variable::named("LEFT_REAR_STEERING_WHEEL_LEFT_ANIM_ANGLE"),
    );

    // Adds rotational speed to bws wheel based on steering angle
    builder.map_many(
        ExecuteOn::PostTick,
        vec![
            Variable::named("REAR_WHEEL_STEERING_POSITION"),
            Variable::aircraft("RIGHT WHEEL ROTATION ANGLE", "degree", 0),
        ],
        |values| {
            normalise_angle(values[1] + (values[0] - 0.5) * REAR_STEERING_RATIO_TO_WHEEL_ANGLE_GAIN)
        },
        Variable::named("RIGHT_REAR_STEERING_WHEEL_RIGHT_ANIM_ANGLE"),
    );
    builder.map_many(
        ExecuteOn::PostTick,
        vec![
            Variable::named("REAR_WHEEL_STEERING_POSITION"),
            Variable::aircraft("RIGHT WHEEL ROTATION ANGLE", "degree", 0),
        ],
        |values| {
            normalise_angle(values[1] - (values[0] - 0.5) * REAR_STEERING_RATIO_TO_WHEEL_ANGLE_GAIN)
        },
        Variable::named("RIGHT_REAR_STEERING_WHEEL_LEFT_ANIM_ANGLE"),
    );

    Ok(())
}

fn recenter_when_close_to_center(value: f64, increment: f64) -> f64 {
    if value < 0.5 + increment && value > 0.5 - increment {
        0.5
    } else {
        value
    }
}

const MAX_CONTROLLABLE_STEERING_ANGLE_DEGREES: f64 = 70.;

fn steering_animation_to_msfs_from_steering_angle(nose_wheel_position: f64) -> f64 {
    const STEERING_ANIMATION_TOTAL_RANGE_DEGREES: f64 = 140.;

    ((nose_wheel_position * MAX_CONTROLLABLE_STEERING_ANGLE_DEGREES
        / (STEERING_ANIMATION_TOTAL_RANGE_DEGREES / 2.))
        / 2.)
        + 0.5
}

fn steering_demand_to_msfs_from_steering_angle(
    nose_wheel_position: f64,
    rudder_position: f64,
) -> f64 {
    const MAX_MSFS_STEERING_ANGLE_DEGREES: f64 = 70.;

    // Steering in msfs is the max we want rescaled to the max in msfs
    let steering_ratio_converted = nose_wheel_position * MAX_CONTROLLABLE_STEERING_ANGLE_DEGREES
        / MAX_MSFS_STEERING_ANGLE_DEGREES
        / 2.
        + 0.5;

    // Steering demand is reverted in msfs so we do 1 - angle.
    // Then we hack msfs by adding the rudder value that it will always substract internally
    // This way we end up with actual angle we required
    (1. - steering_ratio_converted) + (rudder_position - 0.5)
}

fn steering_max_demand_to_msfs_from_steering_angle(nose_wheel_position: f64) -> f64 {
    const MAX_MSFS_STEERING_ANGLE_DEGREES: f64 = 180.;

    // Steering in msfs is the max we want rescaled to the max in msfs
    nose_wheel_position.abs() * MAX_CONTROLLABLE_STEERING_ANGLE_DEGREES
        / MAX_MSFS_STEERING_ANGLE_DEGREES
        / 2.
        + 0.5
}

fn normalise_angle(angle: f64) -> f64 {
    let raw = angle % 360.;

    if raw > 0. {
        raw
    } else {
        raw + 360.
    }
}
