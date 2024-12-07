use wasm_bindgen::prelude::*;
use nu_protocol::engine::{EngineState, Stack};

pub mod engine_state;
pub mod stack;

#[wasm_bindgen]
pub fn execute(code: &str, engine_state: *mut EngineState, stack: *mut Stack) {
    
}
