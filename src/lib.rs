use wasm_bindgen::prelude::*;
use console_error_panic_hook;
use serde::{Serialize, Deserialize};
use tsify_next::Tsify;

mod engine_state;
mod stack;
mod value;
mod error;

pub use engine_state::*;
pub use stack::*;
pub use value::*;
pub use error::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("called once here");
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteOptions {
    name: Option<String>,
    merge_delta: Option<bool>,
    input: Option<Value>,
}

/// Some docs.
/// 
/// @throws lol
#[wasm_bindgen]
pub fn execute(
    code: &str, 
    #[wasm_bindgen(js_name = "engineState")]
    engine_state: &mut EngineState, 
    stack: &mut Stack,
    options: Option<ExecuteOptions>,
) {
    todo!()
}

#[wasm_bindgen]
pub fn yeet() -> TryFromValueError {
    let error = error::TryFromValueError::new("wow".to_string(), js_sys::Object::new());
    error
}
