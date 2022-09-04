mod types;
mod traits;
mod error;
mod actor;
mod util;
mod state;
mod blockstore;

use fvm_sdk::NO_DATA_BLOCK_ID;
use crate::error::Error;
use crate::actor::Actor;
use crate::traits::UptimeCheckerActor;
use crate::util::parse_params_or_abort;

/// The actor's WASM entrypoint. It takes the ID of the parameters block,
/// and returns the ID of the return value block, or NO_DATA_BLOCK_ID if no
/// return value.
#[no_mangle]
pub fn invoke(params_block_id: u32) -> u32 {
    let params = match fvm_sdk::message::params_raw(params_block_id) {
        Ok(v) => v.1,
        Err(e) => fvm_sdk::vm::abort(e as u32, Option::from(e.to_string().as_str()))
    };

    let r = match fvm_sdk::message::method_number() {
        0 => Actor::init(parse_params_or_abort(&params)),
        1 => Actor::new_checker(parse_params_or_abort(&params)),
        2 => Actor::new_member(parse_params_or_abort(&params)),
        3 => Actor::edit_checker(parse_params_or_abort(&params)),
        4 => Actor::edit_member(parse_params_or_abort(&params)),
        5 => Actor::rm_checker(parse_params_or_abort(&params)),
        6 => Actor::rm_member(parse_params_or_abort(&params)),
        7 => Actor::report_checker(parse_params_or_abort(&params)),
        _ => Ok(())
    };

    match r {
        Ok(_) => NO_DATA_BLOCK_ID,
        Err(e) => fvm_sdk::vm::abort(e.code(), None)
    }
}
