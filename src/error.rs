use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/error.js")]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Error)]
    pub type NuJsError;
    #[wasm_bindgen(constructor)]
    pub fn new(message: String) -> NuJsError;

    #[wasm_bindgen(extends = NuJsError, extends = js_sys::Error)]
    pub type TryFromValueError;
    #[wasm_bindgen(constructor)]
    pub fn new(message: String, value: js_sys::Object) -> TryFromValueError;

    #[wasm_bindgen(extends = NuJsError, extends = js_sys::Error)]
    pub type UnsupportedValueError;
    #[wasm_bindgen(constructor)]
    pub fn new(message: String) -> UnsupportedValueError;

    #[wasm_bindgen(extends = NuJsError, extends = js_sys::Error)]
    pub type ParseError;
    #[wasm_bindgen(constructor)]
    pub fn new(message: String) -> ParseError;

    #[wasm_bindgen(extends = NuJsError, extends = js_sys::Error)]
    pub type CompileError;
    #[wasm_bindgen(constructor)]
    pub fn new(message: String) -> CompileError;

    #[wasm_bindgen(extends = NuJsError, extends = js_sys::Error)]
    pub type MergeDeltaError;
    #[wasm_bindgen(constructor)]
    pub fn new(message: String) -> MergeDeltaError;

    #[wasm_bindgen(extends = NuJsError, extends = js_sys::Error)]
    pub type EvalError;
    #[wasm_bindgen(constructor)]
    pub fn new(message: String) -> EvalError;

    #[wasm_bindgen(extends = NuJsError, extends = js_sys::Error)]
    pub type CollectResultsError;
    #[wasm_bindgen(constructor)]
    pub fn new(message: String) -> CollectResultsError;
}

#[wasm_bindgen(typescript_custom_section)]
const ERROR_TYPES: &'static str = include_str!("../js/error.d.ts");
