use wasm_bindgen::prelude::*;
use console_error_panic_hook;

mod engine_state;
mod stack;
mod value;

pub use engine_state::*;
pub use stack::*;
pub use value::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("called once here");
}

#[wasm_bindgen]
pub fn execute(
    code: &str, 
    engine_state: &mut EngineState, 
    stack: &mut Stack,
    name: Option<String>,
    merge_delta: bool,
) {
    todo!()
}
