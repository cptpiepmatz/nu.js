use std::ops::{Deref, DerefMut};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EngineState {
    pub(crate) ptr: *mut nu_protocol::engine::EngineState,
}

#[wasm_bindgen]
impl EngineState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> EngineState {
        let ptr = Box::new(nu_protocol::engine::EngineState::new());
        EngineState {
            ptr: Box::into_raw(ptr),
        }
    }
}

impl Drop for EngineState {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.ptr)) }
    }
}

impl Deref for EngineState {
    type Target = nu_protocol::engine::EngineState;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl DerefMut for EngineState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}
