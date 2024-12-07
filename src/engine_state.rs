use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EngineState {
    pub(crate) ptr: *mut nu_protocol::engine::EngineState
}

#[wasm_bindgen]
impl EngineState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> EngineState {
        let ptr = Box::new(nu_protocol::engine::EngineState::new());
        EngineState {
            ptr: Box::into_raw(ptr)
        }    
    }
}

impl Drop for EngineState {
    fn drop(&mut self) {
        unsafe {drop(Box::from_raw(self.ptr))}
    }
}
