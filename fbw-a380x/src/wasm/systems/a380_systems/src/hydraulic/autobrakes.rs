use systems::{
    hydraulic::brake_circuit::AutobrakeDecelerationGovernor,
    overhead::PressSingleSignalButton,
    shared::{
        interpolation, DelayedPulseTrueLogicGate, DelayedTrueLogicGate, ElectricalBusType,
        ElectricalBuses, LgciuInterface,
    },
    simulation::{
        InitContext, Read, Reader, SimulationElement, SimulationElementVisitor, SimulatorReader,
        SimulatorWriter, UpdateContext, VariableIdentifier, Write,
    },
};

use std::time::Duration;
use uom::si::{
    acceleration::meter_per_second_squared,
    f64::*,
    length::meter,
    ratio::{percent, ratio},
    velocity::{knot, meter_per_second},
};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum A380AutobrakeKnobPosition {
    DISARM = 0,
    BTV = 1,
    LOW = 2,
    L2 = 3,
    L3 = 4,
    HIGH = 5,
}
impl From<f64> for A380AutobrakeKnobPosition {
    fn from(value: f64) -> Self {
        match value as u8 {
            0 => A380AutobrakeKnobPosition::DISARM,
            1 => A380AutobrakeKnobPosition::BTV,
            2 => A380AutobrakeKnobPosition::LOW,
            3 => A380AutobrakeKnobPosition::L2,
            4 => A380AutobrakeKnobPosition::L3,
            5 => A380AutobrakeKnobPosition::HIGH,
            _ => A380AutobrakeKnobPosition::DISARM,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum A380AutobrakeMode {
    DISARM = 0,
    BTV = 1,
    LOW = 2,
    L2 = 3,
    L3 = 4,
    HIGH = 5,

    RTO = 6,
}
impl From<f64> for A380AutobrakeMode {
    fn from(value: f64) -> Self {
        match value as u8 {
            0 => A380AutobrakeMode::DISARM,
            1 => A380AutobrakeMode::BTV,
            2 => A380AutobrakeMode::LOW,
            3 => A380AutobrakeMode::L2,
            4 => A380AutobrakeMode::L3,
            5 => A380AutobrakeMode::HIGH,

            6 => A380AutobrakeMode::RTO,
            _ => A380AutobrakeMode::DISARM,
        }
    }
}

pub struct A380AutobrakePanel {
    selected_mode_id: VariableIdentifier,

    selected_mode: A380AutobrakeKnobPosition,
    rto_button: PressSingleSignalButton,

    mode_has_changed: bool,
}
impl A380AutobrakePanel {
    pub fn new(context: &mut InitContext) -> A380AutobrakePanel {
        A380AutobrakePanel {
            selected_mode_id: context.get_identifier("AUTOBRAKES_SELECTED_MODE".to_owned()),

            selected_mode: A380AutobrakeKnobPosition::DISARM,
            rto_button: PressSingleSignalButton::new(context, "AUTOBRK_RTO_ARM"),

            mode_has_changed: true,
        }
    }

    pub fn selected_mode(&self) -> A380AutobrakeKnobPosition {
        self.selected_mode
    }

    pub fn selected_mode_has_changed(&self) -> bool {
        self.mode_has_changed
    }

    pub fn rto_pressed(&self) -> bool {
        self.rto_button.is_pressed()
    }
}
impl SimulationElement for A380AutobrakePanel {
    fn accept<T: SimulationElementVisitor>(&mut self, visitor: &mut T) {
        self.rto_button.accept(visitor);

        visitor.visit(self);
    }

    fn read(&mut self, reader: &mut SimulatorReader) {
        let raw_read: f64 = reader.read(&self.selected_mode_id);
        let new_mode: A380AutobrakeKnobPosition = raw_read.into();

        self.mode_has_changed = self.selected_mode != new_mode;

        self.selected_mode = new_mode;
    }
}

struct A380AutobrakeKnobSelectorSolenoid {
    disarm_knob_id: VariableIdentifier,

    powered_by: ElectricalBusType,
    is_powered: bool,

    disarm_request: bool,
}
impl A380AutobrakeKnobSelectorSolenoid {
    fn new(context: &mut InitContext, powered_by: ElectricalBusType) -> Self {
        Self {
            disarm_knob_id: context.get_identifier("AUTOBRAKES_DISARM_KNOB_REQ".to_owned()),

            powered_by,
            is_powered: true,

            disarm_request: false,
        }
    }

    fn disarm(&mut self, solenoid_should_disarm: bool) {
        self.disarm_request = self.is_powered && solenoid_should_disarm;
    }
}
impl SimulationElement for A380AutobrakeKnobSelectorSolenoid {
    fn write(&self, writer: &mut SimulatorWriter) {
        writer.write(&self.disarm_knob_id, self.disarm_request);
    }

    fn receive_power(&mut self, buses: &impl ElectricalBuses) {
        self.is_powered = buses.is_powered(self.powered_by)
    }
}

/// Autobrake controller computes the state machine of the autobrake logic, and the deceleration target
/// that we expect for the plane
pub struct A380AutobrakeController {
    armed_mode_id: VariableIdentifier,
    decel_light_id: VariableIdentifier,
    active_id: VariableIdentifier,
    rto_mode_armed_id: VariableIdentifier,

    external_disarm_event_id: VariableIdentifier,

    deceleration_governor: AutobrakeDecelerationGovernor,
    decelerating_light: bool,

    target: Acceleration,
    mode: A380AutobrakeMode,

    arming_is_allowed_by_bcu: bool,
    left_brake_pedal_input: Ratio,
    right_brake_pedal_input: Ratio,

    ground_spoilers_are_deployed: bool,
    last_ground_spoilers_are_deployed: bool,
    ground_spoilers_are_deployed_since_5s: DelayedTrueLogicGate,
    nose_gear_was_compressed_once: bool,

    should_disarm_after_time_in_flight: DelayedPulseTrueLogicGate,
    should_reject_rto_mode_after_time_in_flight: DelayedTrueLogicGate,

    autobrake_knob: A380AutobrakeKnobSelectorSolenoid,

    external_disarm_event: bool,

    placeholder_ground_spoilers_out: bool,

    btv_scheduler: BtvDecelScheduler,

    // This delay is added only so you have time to click the knob pass the BTV position without an unprogrammed BTV sending knob back to disarm
    should_disarm_selection_knob_delayed: DelayedTrueLogicGate,
}
impl A380AutobrakeController {
    const DURATION_OF_FLIGHT_TO_DISARM_AUTOBRAKE: Duration = Duration::from_secs(10);
    const DURATION_OF_GROUND_SPOILERS_BEFORE_ARMING: Duration = Duration::from_secs(5);

    // Time breakpoint map is shared by all normal modes, and there's a BTV placeholder delaying braking
    const NORMAL_MODE_DECEL_PROFILE_TIME_S: [f64; 3] = [0., 0.1, 2.5];

    const LOW_MODE_DECEL_PROFILE_ACCEL_MS2: [f64; 3] = [4., 0., -2.];
    const L2_MODE_DECEL_PROFILE_ACCEL_MS2: [f64; 3] = [4., 0., -2.5];
    const L3_MODE_DECEL_PROFILE_ACCEL_MS2: [f64; 3] = [4., 0., -3.];
    const HIGH_MODE_DECEL_PROFILE_ACCEL_MS2: [f64; 3] = [4., -2., -3.5];

    const RTO_MODE_DECEL_TARGET_MS2: f64 = -6.;
    const OFF_MODE_DECEL_TARGET_MS2: f64 = 5.;

    const MARGIN_PERCENT_TO_TARGET_TO_SHOW_DECEL_IN_LANDING_MODE: f64 = 80.;
    const MARGIN_PERCENT_TO_TARGET_TO_REMOVE_DECEL_IN_LANDING_MODE: f64 = 70.;
    const TARGET_TO_SHOW_DECEL_IN_RTO_MS2: f64 = -2.7;
    const TARGET_TO_REMOVE_DECEL_IN_RTO_MS2: f64 = -2.;

    pub fn new(context: &mut InitContext) -> A380AutobrakeController {
        A380AutobrakeController {
            armed_mode_id: context.get_identifier("AUTOBRAKES_ARMED_MODE".to_owned()),
            decel_light_id: context.get_identifier("AUTOBRAKES_DECEL_LIGHT".to_owned()),
            active_id: context.get_identifier("AUTOBRAKES_ACTIVE".to_owned()),
            rto_mode_armed_id: context.get_identifier("AUTOBRAKES_RTO_ARMED".to_owned()),

            external_disarm_event_id: context.get_identifier("AUTOBRAKE_DISARM".to_owned()),

            deceleration_governor: AutobrakeDecelerationGovernor::new(),
            decelerating_light: false,
            target: Acceleration::new::<meter_per_second_squared>(0.),
            mode: A380AutobrakeMode::DISARM,
            arming_is_allowed_by_bcu: context.is_in_flight(),
            left_brake_pedal_input: Ratio::new::<percent>(0.),
            right_brake_pedal_input: Ratio::new::<percent>(0.),
            ground_spoilers_are_deployed: false,
            last_ground_spoilers_are_deployed: false,
            ground_spoilers_are_deployed_since_5s: DelayedTrueLogicGate::new(
                Self::DURATION_OF_GROUND_SPOILERS_BEFORE_ARMING,
            ),
            nose_gear_was_compressed_once: false,
            should_disarm_after_time_in_flight: DelayedPulseTrueLogicGate::new(
                Self::DURATION_OF_FLIGHT_TO_DISARM_AUTOBRAKE,
            )
            .starting_as(context.is_in_flight(), false),
            should_reject_rto_mode_after_time_in_flight: DelayedTrueLogicGate::new(
                Self::DURATION_OF_FLIGHT_TO_DISARM_AUTOBRAKE,
            )
            .starting_as(context.is_in_flight()),

            // Powered on VDC BUS 2 -> 806GG cb
            autobrake_knob: A380AutobrakeKnobSelectorSolenoid::new(
                context,
                ElectricalBusType::DirectCurrent(2),
            ),

            external_disarm_event: false,

            placeholder_ground_spoilers_out: false,

            btv_scheduler: BtvDecelScheduler::new(context),

            should_disarm_selection_knob_delayed: DelayedTrueLogicGate::new(Duration::from_millis(
                500,
            )),
        }
    }

    fn spoilers_retracted_during_this_update(&self) -> bool {
        !self.ground_spoilers_are_deployed && self.last_ground_spoilers_are_deployed
    }

    fn rto_mode_deselected_this_update(&self, autobrake_panel: &A380AutobrakePanel) -> bool {
        self.mode == A380AutobrakeMode::RTO && autobrake_panel.rto_pressed()
    }

    pub fn brake_output(&self) -> Ratio {
        Ratio::new::<ratio>(self.deceleration_governor.output())
    }

    fn determine_mode(&mut self, autobrake_panel: &A380AutobrakePanel) -> A380AutobrakeMode {
        if self.mode != A380AutobrakeMode::RTO
            && autobrake_panel.rto_pressed()
            && !self.should_reject_rto_mode_after_time_in_flight.output()
        {
            A380AutobrakeMode::RTO
        } else {
            if autobrake_panel.selected_mode_has_changed() {
                match autobrake_panel.selected_mode() {
                    A380AutobrakeKnobPosition::DISARM => A380AutobrakeMode::DISARM,
                    A380AutobrakeKnobPosition::LOW => A380AutobrakeMode::LOW,
                    A380AutobrakeKnobPosition::L2 => A380AutobrakeMode::L2,
                    A380AutobrakeKnobPosition::L3 => A380AutobrakeMode::L3,
                    A380AutobrakeKnobPosition::HIGH => A380AutobrakeMode::HIGH,
                    A380AutobrakeKnobPosition::BTV => {
                        self.btv_scheduler.enable();
                        A380AutobrakeMode::BTV
                    }
                }
            } else {
                self.mode
            }
        }
    }

    fn should_engage_deceleration_governor(
        &self,
        context: &UpdateContext,
        autobrake_panel: &A380AutobrakePanel,
    ) -> bool {
        self.is_armed()
            && self.ground_spoilers_are_deployed // We wait 5s after deploy, but they need to be deployed even if nose compressed
            && (self.ground_spoilers_are_deployed_since_5s.output()
                || self.nose_gear_was_compressed_once)
            && !self.should_disarm(context, autobrake_panel)
    }

    fn is_armed(&self) -> bool {
        self.mode != A380AutobrakeMode::DISARM
    }

    fn is_decelerating(&self) -> bool {
        self.decelerating_light
    }

    /// Handles the hysteresis for decel light depending on normal vs RTO modes
    fn update_decelerating_light_info(&mut self) {
        if !self.deceleration_demanded() {
            self.decelerating_light = false;
            return;
        }

        match self.mode {
            A380AutobrakeMode::DISARM => self.decelerating_light = false,
            A380AutobrakeMode::LOW
            | A380AutobrakeMode::L2
            | A380AutobrakeMode::L3
            | A380AutobrakeMode::HIGH
            | A380AutobrakeMode::BTV => {
                if self
                    .deceleration_governor
                    .is_on_target(Ratio::new::<percent>(
                        Self::MARGIN_PERCENT_TO_TARGET_TO_SHOW_DECEL_IN_LANDING_MODE,
                    ))
                {
                    self.decelerating_light = true;
                } else if !self
                    .deceleration_governor
                    .is_on_target(Ratio::new::<percent>(
                        Self::MARGIN_PERCENT_TO_TARGET_TO_REMOVE_DECEL_IN_LANDING_MODE,
                    ))
                {
                    self.decelerating_light = false;
                }
            }
            A380AutobrakeMode::RTO => {
                if self
                    .deceleration_governor
                    .decelerating_at_or_above_rate(Acceleration::new::<meter_per_second_squared>(
                        Self::TARGET_TO_SHOW_DECEL_IN_RTO_MS2,
                    ))
                {
                    self.decelerating_light = true;
                } else if !self.deceleration_governor.decelerating_at_or_above_rate(
                    Acceleration::new::<meter_per_second_squared>(
                        Self::TARGET_TO_REMOVE_DECEL_IN_RTO_MS2,
                    ),
                ) {
                    self.decelerating_light = false;
                }
            }
        }
    }

    fn deceleration_demanded(&self) -> bool {
        self.deceleration_governor.is_engaged()
            && self.target.get::<meter_per_second_squared>() < 0.
    }

    fn should_disarm_due_to_pedal_input(&self) -> bool {
        // Thresholds from A320, TBC for A380
        match self.mode {
            A380AutobrakeMode::DISARM => false,
            A380AutobrakeMode::LOW
            | A380AutobrakeMode::L2
            | A380AutobrakeMode::L3
            | A380AutobrakeMode::HIGH
            | A380AutobrakeMode::BTV => {
                self.left_brake_pedal_input > Ratio::new::<percent>(53.)
                    || self.right_brake_pedal_input > Ratio::new::<percent>(53.)
                    || (self.left_brake_pedal_input > Ratio::new::<percent>(11.)
                        && self.right_brake_pedal_input > Ratio::new::<percent>(11.))
            }
            A380AutobrakeMode::RTO => {
                self.left_brake_pedal_input > Ratio::new::<percent>(77.)
                    || self.right_brake_pedal_input > Ratio::new::<percent>(77.)
                    || (self.left_brake_pedal_input > Ratio::new::<percent>(53.)
                        && self.right_brake_pedal_input > Ratio::new::<percent>(53.))
            }
        }
    }

    fn should_disarm(&self, context: &UpdateContext, autobrake_panel: &A380AutobrakePanel) -> bool {
        // when a simulation is started in flight, some values need to be ignored for a certain time to ensure
        // an unintended disarm is not happening
        (self.deceleration_governor.is_engaged() && self.should_disarm_due_to_pedal_input())
            || (context.is_sim_ready() && !self.arming_is_allowed_by_bcu)
            || self.spoilers_retracted_during_this_update()
            || self.should_disarm_after_time_in_flight.output()
            || (self.external_disarm_event && self.mode != A380AutobrakeMode::RTO)
            || (self.mode == A380AutobrakeMode::RTO
                && self.should_reject_rto_mode_after_time_in_flight.output())
            || (self.mode == A380AutobrakeMode::DISARM
                && autobrake_panel.selected_mode() != A380AutobrakeKnobPosition::DISARM)
          // || (self.mode == A380AutobrakeMode::BTV && !self.btv_scheduler.arming_authorized())
            || (self.mode == A380AutobrakeMode::BTV && !self.btv_scheduler.is_armed())
    }

    fn disarm_actions(&mut self) {
        self.btv_scheduler.disarm();
        self.nose_gear_was_compressed_once = false;
        self.mode = A380AutobrakeMode::DISARM;
    }

    fn calculate_target(&mut self) -> Acceleration {
        Acceleration::new::<meter_per_second_squared>(match self.mode {
            A380AutobrakeMode::DISARM => Self::OFF_MODE_DECEL_TARGET_MS2,
            A380AutobrakeMode::LOW => interpolation(
                &Self::NORMAL_MODE_DECEL_PROFILE_TIME_S,
                &Self::LOW_MODE_DECEL_PROFILE_ACCEL_MS2,
                self.deceleration_governor.time_engaged().as_secs_f64(),
            ),
            A380AutobrakeMode::L2 => interpolation(
                &Self::NORMAL_MODE_DECEL_PROFILE_TIME_S,
                &Self::L2_MODE_DECEL_PROFILE_ACCEL_MS2,
                self.deceleration_governor.time_engaged().as_secs_f64(),
            ),
            A380AutobrakeMode::L3 => interpolation(
                &Self::NORMAL_MODE_DECEL_PROFILE_TIME_S,
                &Self::L3_MODE_DECEL_PROFILE_ACCEL_MS2,
                self.deceleration_governor.time_engaged().as_secs_f64(),
            ),
            A380AutobrakeMode::HIGH => interpolation(
                &Self::NORMAL_MODE_DECEL_PROFILE_TIME_S,
                &Self::HIGH_MODE_DECEL_PROFILE_ACCEL_MS2,
                self.deceleration_governor.time_engaged().as_secs_f64(),
            ),
            A380AutobrakeMode::BTV => self.compute_btv_decel_target_ms2(),
            A380AutobrakeMode::RTO => Self::RTO_MODE_DECEL_TARGET_MS2,
        })
    }

    fn compute_btv_decel_target_ms2(&self) -> f64 {
        self.btv_scheduler.decel().get::<meter_per_second_squared>()
    }

    fn update_input_conditions(
        &mut self,
        context: &UpdateContext,
        allow_arming: bool,
        pedal_input_left: Ratio,
        pedal_input_right: Ratio,
        lgciu1: &impl LgciuInterface,
        lgciu2: &impl LgciuInterface,
    ) {
        let in_flight_lgciu1 =
            !lgciu1.right_gear_compressed(false) && !lgciu1.left_gear_compressed(false);
        let in_flight_lgciu2 =
            !lgciu2.right_gear_compressed(false) && !lgciu2.left_gear_compressed(false);

        // Stays true until disarming
        self.nose_gear_was_compressed_once = self.nose_gear_was_compressed_once
            || lgciu1.nose_gear_compressed(false)
            || lgciu2.nose_gear_compressed(false);

        self.ground_spoilers_are_deployed_since_5s
            .update(context, self.ground_spoilers_are_deployed);
        self.should_disarm_after_time_in_flight
            .update(context, in_flight_lgciu1 && in_flight_lgciu2);
        self.should_reject_rto_mode_after_time_in_flight
            .update(context, in_flight_lgciu1 && in_flight_lgciu2);

        self.arming_is_allowed_by_bcu = allow_arming;
        self.left_brake_pedal_input = pedal_input_left;
        self.right_brake_pedal_input = pedal_input_right;
    }

    pub fn update(
        &mut self,
        context: &UpdateContext,
        autobrake_panel: &A380AutobrakePanel,
        allow_arming: bool,
        pedal_input_left: Ratio,
        pedal_input_right: Ratio,
        lgciu1: &impl LgciuInterface,
        lgciu2: &impl LgciuInterface,
        placeholder_ground_spoilers_out: bool,
    ) {
        self.update_input_conditions(
            context,
            allow_arming,
            pedal_input_left,
            pedal_input_right,
            lgciu1,
            lgciu2,
        );

        let rto_disable = self.rto_mode_deselected_this_update(autobrake_panel);

        //println!("PRE MOD AFFECTATION => {:?}", self.mode);
        self.mode = self.determine_mode(autobrake_panel);
        //println!("AFTER MOD AFFECTATION => {:?}", self.mode);

        if rto_disable || self.should_disarm(context, autobrake_panel) {
            self.disarm_actions();
        }

        self.should_disarm_selection_knob_delayed.update(
            context,
            self.mode == A380AutobrakeMode::DISARM || self.mode == A380AutobrakeMode::RTO,
        );

        self.autobrake_knob
            .disarm(self.should_disarm_selection_knob_delayed.output());

        self.deceleration_governor
            .engage_when(self.should_engage_deceleration_governor(context, autobrake_panel));

        self.target = self.calculate_target();
        self.deceleration_governor.update(context, self.target);
        self.update_decelerating_light_info();

        self.placeholder_ground_spoilers_out = placeholder_ground_spoilers_out;

        self.btv_scheduler
            .update(context, self.ground_spoilers_are_deployed);
    }
}
impl SimulationElement for A380AutobrakeController {
    fn accept<T: SimulationElementVisitor>(&mut self, visitor: &mut T) {
        self.autobrake_knob.accept(visitor);
        self.btv_scheduler.accept(visitor);

        visitor.visit(self);
    }

    fn write(&self, writer: &mut SimulatorWriter) {
        writer.write(&self.armed_mode_id, self.mode as u8 as f64);
        writer.write(&self.decel_light_id, self.is_decelerating());
        writer.write(&self.active_id, self.deceleration_demanded());
        writer.write(&self.rto_mode_armed_id, self.mode == A380AutobrakeMode::RTO);
    }

    fn read(&mut self, reader: &mut SimulatorReader) {
        self.last_ground_spoilers_are_deployed = self.ground_spoilers_are_deployed;
        self.ground_spoilers_are_deployed = self.placeholder_ground_spoilers_out;

        self.external_disarm_event = reader.read(&self.external_disarm_event_id);

        // Reading current mode in sim to initialize correct mode if sim changes it (from .FLT files for example)
        let readed_mode = reader.read_f64(&self.armed_mode_id);
        if readed_mode >= 0.0 {
            self.mode = readed_mode.into();
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum BTVState {
    Disabled,
    Armed,
    RotOptimization,
    Decel,
    EndOfBraking,
    OutOfDecelRange,
}

struct BtvDecelScheduler {
    dev_exit_distance_to_touchdown_id: VariableIdentifier,
    runway_length_id: VariableIdentifier,
    distance_to_exit_id: VariableIdentifier,

    ground_speed_id: VariableIdentifier,

    runway_length: Length,

    rolling_distance: Length,
    fallback_distance_to_exit_from_touchdown: Length,
    oans_distance_to_exit: Length,

    spoilers_active: bool,

    ground_speed: Velocity,

    state: BTVState,

    deceleration_request: Acceleration,
    end_of_decel_acceleration: Acceleration,

    final_distance_remaining: Length,

    distance_remaining_at_decel_activation: Length,
}
impl BtvDecelScheduler {
    const MAX_DECEL_DRY_MS2: f64 = -3.;
    const MAX_DECEL_WET_MS2: f64 = -2.;

    const MIN_RUNWAY_LENGTH_M: f64 = 1500.;

    const DISTANCE_OFFSET_TO_RELEASE_BTV: f64 = 50.;
    const TARGET_SPEED_TO_RELEASE_BTV: f64 = 5.15;

    fn new(context: &mut InitContext) -> Self {
        Self {
            dev_exit_distance_to_touchdown_id: context
                .get_identifier("OANS_BTV_REQ_STOPPING_DISTANCE".to_owned()),
            runway_length_id: context.get_identifier("OANS_RWY_LENGTH".to_owned()),
            distance_to_exit_id: context
                .get_identifier("OANS_BTV_REMAINING_DIST_TO_EXIT".to_owned()),

            ground_speed_id: context.get_identifier("GPS GROUND SPEED".to_owned()),

            runway_length: Length::default(),
            rolling_distance: Length::default(),
            fallback_distance_to_exit_from_touchdown: Length::default(),
            oans_distance_to_exit: Length::default(),

            spoilers_active: false,
            ground_speed: Velocity::default(),

            state: BTVState::Disabled,

            deceleration_request: Acceleration::default(),
            end_of_decel_acceleration: Acceleration::default(),

            final_distance_remaining: Length::default(),

            distance_remaining_at_decel_activation: Length::default(),
        }
    }

    fn enable(&mut self) {
        if self.state == BTVState::Disabled && self.arming_authorized() {
            self.state = BTVState::Armed;
        }
    }

    fn disarm(&mut self) {
        self.state = BTVState::Disabled;
        self.deceleration_request = Acceleration::new::<meter_per_second_squared>(5.);
        self.end_of_decel_acceleration = Acceleration::new::<meter_per_second_squared>(5.);
        self.final_distance_remaining = Length::default();
        self.distance_remaining_at_decel_activation = Length::default();
    }

    fn decel(&self) -> Acceleration {
        match self.state {
            BTVState::Decel | BTVState::OutOfDecelRange => self.deceleration_request,
            BTVState::EndOfBraking => self.end_of_decel_acceleration,
            BTVState::RotOptimization => self.accel_during_rot_opti(),
            _ => Acceleration::new::<meter_per_second_squared>(5.),
        }
    }

    fn update(&mut self, context: &UpdateContext, spoilers_active: bool) {
        self.spoilers_active = spoilers_active;
        self.integrate_distance(context);

        println!(
            "BTV MODE {:?}, roll distance {:.0}m, remaining_dist {:.1} m, speed {:.1} knot",
            self.state,
            self.rolling_distance.get::<meter>(),
            self.braking_distance_remaining().get::<meter>().max(0.),
            self.ground_speed.get::<knot>()
        );

        self.compute_decel(context);

        self.state = self.update_state(context);
    }

    fn braking_distance_remaining(&self) -> Length {
        let distance_remaining_raw = if self.is_oans_fallback_mode() {
            self.fallback_distance_to_exit_from_touchdown - self.rolling_distance
        } else {
            self.oans_distance_to_exit
        };

        let distance_from_btv_exit = Length::new::<meter>(Self::DISTANCE_OFFSET_TO_RELEASE_BTV);

        (distance_remaining_raw - distance_from_btv_exit).max(Length::default())
    }

    fn compute_decel(&mut self, context: &UpdateContext) {
        match self.state {
            BTVState::RotOptimization
            | BTVState::Decel
            | BTVState::EndOfBraking
            | BTVState::OutOfDecelRange => {
                let speed_at_btv_release =
                    Velocity::new::<meter_per_second>(Self::TARGET_SPEED_TO_RELEASE_BTV) * 0.9; // 10% safety margin on release speed

                self.final_distance_remaining = self.braking_distance_remaining();

                let delta_speed_to_achieve = self.ground_speed - speed_at_btv_release;

                let target_deceleration_raw =
                    -delta_speed_to_achieve.get::<meter_per_second>().powi(2)
                        / (2. * self.final_distance_remaining.get::<meter>());

                let target_deceleration_safety_corrected =
                    target_deceleration_raw * self.safety_margin();

                let decel_governor_error = target_deceleration_safety_corrected
                    - context.long_accel().get::<meter_per_second_squared>();

                println!(
                    "deltaV ms ==> {:.2}  Remaining meters {:.0}  ==> Target Decel {:.2} Final target {:.2} true decel {:.2}",
                    delta_speed_to_achieve.get::<meter_per_second>(),
                    self.final_distance_remaining.get::<meter>(),

                    target_deceleration_raw,
                    target_deceleration_safety_corrected,
                    context.long_accel().get::<meter_per_second_squared>()
                );
                println!("GOVERNOR ERROR {:.2}", decel_governor_error);

                self.deceleration_request = Acceleration::new::<meter_per_second_squared>(
                    target_deceleration_safety_corrected
                        .max(Self::MAX_DECEL_DRY_MS2)
                        .min(5.),
                );
            }
            _ => {
                self.deceleration_request = Acceleration::new::<meter_per_second_squared>(5.);
            }
        }
    }

    fn arming_authorized(&self) -> bool {
        self.runway_length.get::<meter>() >= Self::MIN_RUNWAY_LENGTH_M
            && (self.oans_distance_to_exit.get::<meter>() > -1.
                || self.fallback_distance_to_exit_from_touchdown.get::<meter>() > -1.)
    }

    fn accel_to_reach_to_decelerate(&self) -> Acceleration {
        let percent_of_max = 0.6;
        Acceleration::new::<meter_per_second_squared>(Self::MAX_DECEL_DRY_MS2 * percent_of_max)
    }

    fn accel_during_rot_opti(&self) -> Acceleration {
        let percent_of_max = 0.1;
        Acceleration::new::<meter_per_second_squared>(Self::MAX_DECEL_DRY_MS2 * percent_of_max)
    }

    fn safety_margin(&self) -> f64 {
        match self.state {
            BTVState::Decel | BTVState::EndOfBraking | BTVState::OutOfDecelRange => {
                let ratio_of_decel_distance =
                    self.braking_distance_remaining() / self.distance_remaining_at_decel_activation;

                println!(
                    "BRAKING ==> Distance Ratio {:.2} safety margin {:.2}",
                    ratio_of_decel_distance.get::<ratio>(),
                    (1. + (ratio_of_decel_distance.get::<ratio>().sqrt() * 0.4))
                        .max(1.15)
                        .min(1.4)
                );

                (1. + (ratio_of_decel_distance.get::<ratio>().sqrt() * 0.4))
                    .max(1.15)
                    .min(1.4)
            }

            BTVState::Disabled | BTVState::Armed | BTVState::RotOptimization => {
                println!("OPTIM ==> safety margin {:.2}", 1.4);
                1.4
            }
        }
    }

    fn update_state(&mut self, context: &UpdateContext) -> BTVState {
        match self.state {
            BTVState::Armed => {
                if self.spoilers_active {
                    BTVState::RotOptimization
                } else {
                    if !self.arming_authorized() {
                        BTVState::Disabled
                    } else {
                        self.state
                    }
                }
            }
            BTVState::RotOptimization => {
                let accel_min = self.accel_to_reach_to_decelerate();
                println!(
                    "ROT => waiting for accel {:.2}  current target:{:.2} current true {:.2}",
                    accel_min.get::<meter_per_second_squared>(),
                    self.deceleration_request.get::<meter_per_second_squared>(),
                    context.long_accel().get::<meter_per_second_squared>()
                );

                if self.deceleration_request < accel_min {
                    self.distance_remaining_at_decel_activation = self.braking_distance_remaining();
                    self.end_of_decel_acceleration = self.deceleration_request;
                    BTVState::Decel
                } else {
                    self.state
                }
            }
            BTVState::Decel => {
                if self.final_distance_remaining.get::<meter>() < 50.
                    || self.ground_speed.get::<meter_per_second>()
                        <= Self::TARGET_SPEED_TO_RELEASE_BTV
                {
                    BTVState::EndOfBraking
                } else {
                    BTVState::Decel
                }
            }
            BTVState::EndOfBraking => {
                if self.ground_speed.get::<meter_per_second>() <= Self::TARGET_SPEED_TO_RELEASE_BTV
                {
                    self.disarm();
                    BTVState::Disabled
                } else {
                    println!(
                        "END OF DECEL TARGET: real time {:.2} / final {:.2}",
                        self.deceleration_request.get::<meter_per_second_squared>(),
                        self.end_of_decel_acceleration
                            .get::<meter_per_second_squared>(),
                    );
                    self.end_of_decel_acceleration = self
                        .end_of_decel_acceleration
                        .max(self.deceleration_request);
                    BTVState::EndOfBraking
                }
            }

            _ => self.state,
        }
    }

    fn integrate_distance(&mut self, context: &UpdateContext) {
        match self.state {
            BTVState::RotOptimization
            | BTVState::Decel
            | BTVState::EndOfBraking
            | BTVState::OutOfDecelRange => {
                let distance_this_tick = self.ground_speed * context.delta_as_time();
                self.rolling_distance = self.rolling_distance + distance_this_tick;
            }

            BTVState::Disabled | BTVState::Armed => self.rolling_distance = Length::default(),
        }
    }

    fn is_oans_fallback_mode(&self) -> bool {
        self.oans_distance_to_exit.get::<meter>() < 0.
    }

    fn is_armed(&self) -> bool {
        self.state != BTVState::Disabled
    }
}
impl SimulationElement for BtvDecelScheduler {
    fn write(&self, writer: &mut SimulatorWriter) {
        writer.write(
            &self.dev_exit_distance_to_touchdown_id,
            self.fallback_distance_to_exit_from_touchdown.get::<meter>(),
        );
        writer.write(&self.runway_length_id, self.runway_length.get::<meter>());
    }

    fn read(&mut self, reader: &mut SimulatorReader) {
        let fallback_raw_exit_distance_meters =
            reader.read_f64(&self.dev_exit_distance_to_touchdown_id);
        self.fallback_distance_to_exit_from_touchdown =
            Length::new::<meter>(fallback_raw_exit_distance_meters);

        let runway_length_meters = reader.read_f64(&self.runway_length_id);
        self.runway_length = Length::new::<meter>(runway_length_meters);

        let raw_oans_distance_to_exit = reader.read_f64(&self.distance_to_exit_id);
        self.oans_distance_to_exit = Length::new::<meter>(raw_oans_distance_to_exit);

        self.ground_speed = reader.read(&self.ground_speed_id);

        // println!(
        //     "OANS ===> DISTANCE {:.0}  FB {:.0}  IS FB mode {:?}, armingAut {:?}, runwayL {:?}",
        //     self.oans_distance_to_exit.get::<meter>(),
        //     self.fallback_distance_to_exit_from_touchdown.get::<meter>(),
        //     self.is_oans_fallback_mode(),
        //     self.arming_authorized(),
        //     self.runway_length.get::<meter>()
        // );
    }
}
