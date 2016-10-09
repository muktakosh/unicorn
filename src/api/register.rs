//! Register API

use jsonrpc_core::{AsyncMethodCommand, Params, Ready, Value};

pub struct RegisterAPI;

impl AsyncMethodCommand for RegisterAPI {
    fn execute(&self, _: Params, ready: Ready) {
        // TODO: Do actual stuff here
        ready.ready(Ok(Value::String("You are registered".to_string())))
    }
}
