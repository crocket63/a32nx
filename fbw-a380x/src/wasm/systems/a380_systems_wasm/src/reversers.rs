use std::error::Error;

use msfs::sim_connect;
use msfs::{sim_connect::SimConnect, sim_connect::SIMCONNECT_OBJECT_ID_USER};

use systems_wasm::aspects::{ExecuteOn, MsfsAspectBuilder, ObjectWrite, VariablesToObject};
use systems_wasm::{set_data_on_sim_object, Variable};

pub(super) fn reversers(builder: &mut MsfsAspectBuilder) -> Result<(), Box<dyn Error>> {
    // Used for 320 reverser hack. Need it here as well so the 380 sees correct Z acceleration
    builder.map_many(
        ExecuteOn::PreTick,
        vec![
            Variable::aircraft("ACCELERATION BODY Z", "Feet per second squared", 0),
            Variable::aspect("REVERSER_DELTA_ACCEL"),
        ],
        |values| values[0] + values[1],
        Variable::aspect("ACCELERATION_BODY_Z_WITH_REVERSER"),
    );

    Ok(())
}
