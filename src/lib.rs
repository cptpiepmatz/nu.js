use wasm_bindgen::prelude::*;

pub mod engine_state;
pub mod stack;

#[wasm_bindgen]
pub fn execute(code: &str, engine_state: &mut engine_state::EngineState, stack: &mut stack::Stack) {
    
}
