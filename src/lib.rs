use std::ops::DerefMut;

use console_error_panic_hook;
use log::*;
use nu_protocol::{
    debugger::WithoutDebug, engine::StateWorkingSet, PipelineData, Span, Value as NuValue,
};
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

mod engine_state;
mod error;
mod stack;
mod value;

pub use engine_state::*;
pub use error::*;
pub use stack::*;
pub use value::*;

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
/// @throws {TryFromValueError} - Thrown if the passed input type does not adhere to {@link Value}'s definition.
/// @throws {NuJsError}
#[wasm_bindgen]
pub fn execute(
    code: &str,
    #[wasm_bindgen(js_name = "engineState")] engine_state: &mut EngineState,
    stack: &mut Stack,
    #[wasm_bindgen(unchecked_param_type = "ExecuteOptions | undefined")] options: Option<
        js_sys::Object,
    >,
) -> Result<Value, NuJsError> {
    let options = match options {
        None => ExecuteOptions::default(),
        Some(options) => ExecuteOptions::from_js(options)
            .map_err(|_| TryFromValueError::new("dunno".to_string(), js_sys::Object::new()))?,
    };

    let span = nu_protocol::Span::unknown();
    let input = options
        .input
        .map(|input| NuValue::try_from(input))
        .transpose()?;

    let code = code.as_bytes();
    let engine_state = engine_state.deref_mut();
    let mut working_set = StateWorkingSet::new(engine_state);
    let name = options.name.as_ref().map(|name| name.as_str());
    let block = nu_parser::parse(&mut working_set, name, code, false);

    // TODO: report parse warnings

    if let Some(error) = working_set.parse_errors.into_iter().next() {
        return Err(ParseError::new(error.to_string()).into());
    }

    if let Some(error) = working_set.compile_errors.into_iter().next() {
        return Err(CompileError::new(error.to_string()).into());
    }

    if options.merge_delta.unwrap_or(false) {
        engine_state
            .merge_delta(working_set.delta)
            .map_err(|e| MergeDeltaError::new(e.to_string()))?;
    }

    let input = match input {
        None => PipelineData::Empty,
        Some(input) => PipelineData::Value(input, None),
    };

    let res = nu_engine::eval_block::<WithoutDebug>(engine_state, stack, &block, input)
        .map_err(|e| EvalError::new(e.to_string()))?;
    let res = res
        .into_value(Span::unknown())
        .map_err(|e| CollectResultsError::new(e.to_string()))?;
    Ok(res.into())
}

#[wasm_bindgen]
pub fn yeet() -> NuJsError {
    let error = error::NuJsError::new("wow".to_string());
    error
}

mod placeholder {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(skip_typescript, js_name = "__nu_js__reexport__placeholder")]
    pub fn placeholder() {}
}
