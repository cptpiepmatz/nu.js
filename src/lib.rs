use wasm_bindgen::prelude::*;
use console_error_panic_hook;
use serde::{Serialize, Deserialize};
use tsify_next::Tsify;
use nu_protocol::Value as NuValue;
use log::*;

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

#[derive(Serialize, Deserialize, Default, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteOptions {
    #[tsify(optional)]
    name: Option<String>,
    #[tsify(optional)]
    merge_delta: Option<bool>,
    #[tsify(optional)]
    input: Option<Value>,
}

#[allow(unused)]
/// Some docs.
/// 
/// @throws {NuJsError}
#[wasm_bindgen]
pub fn execute(
    code: &str, 
    #[wasm_bindgen(js_name = "engineState")]
    engine_state: &mut EngineState, 
    stack: &mut Stack,
    #[wasm_bindgen(unchecked_param_type = "ExecuteOptions | undefined")]
    options: JsValue,
) -> Result<Value, NuJsError> {
    let options = match options.is_undefined() {
        true => ExecuteOptions::default(),
        false => ExecuteOptions::from_js(options).map_err(|_| TryFromValueError::new("dunno".to_string(), js_sys::Object::new()))?
    };

    let span = nu_protocol::Span::unknown();
    let input: NuValue = match options.input {
        None => NuValue::nothing(span),
        Some(input) => NuValue::try_from(input)?,
    };

    debug!("{input:?}");

    todo!()
}

#[wasm_bindgen]
pub fn yeet() -> TryFromValueError {
    let error = error::TryFromValueError::new("wow".to_string(), js_sys::Object::new());
    error
}
