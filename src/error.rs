use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/error.js")]
extern "C" {

    #[wasm_bindgen]
    pub type TryFromValueError;

    #[wasm_bindgen(constructor)]
    pub fn new(message: String, value: js_sys::Object) -> TryFromValueError;
}

#[wasm_bindgen(typescript_custom_section)]
const ERROR_TYPES: &'static str = include_str!("../js/error.d.ts");

mod placeholder {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(skip_typescript, js_name = "__nu_js__error__placeholder")]
    pub fn placeholder() {}
}
