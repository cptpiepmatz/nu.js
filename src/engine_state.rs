use std::ops::{Deref, DerefMut};
use tsify_next::Tsify;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EngineState {
    pub(crate) ptr: *mut nu_protocol::engine::EngineState,
}

#[derive(Tsify, Debug, Default, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct EngineStateOptions {
    pub default_context: Option<bool>,
    pub shell_command_context: Option<bool>,
    pub extra_command_context: Option<bool>,
}

#[wasm_bindgen]
impl EngineState {
    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<EngineStateOptions>) -> EngineState {
        let options = options.unwrap_or_default();
        
        let engine_state = match options.default_context.unwrap_or(true) {
            true => nu_cmd_lang::create_default_context(),
            false => nu_protocol::engine::EngineState::new(),
        };
        
        let engine_state = match options.shell_command_context.unwrap_or(true) {
            true => nu_command::add_shell_command_context(engine_state),
            false => engine_state,
        };

        let engine_state = match options.extra_command_context.unwrap_or(true) {
            true => nu_cmd_extra::add_extra_command_context(engine_state),
            false => engine_state,
        };

        let ptr = Box::new(engine_state);
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
