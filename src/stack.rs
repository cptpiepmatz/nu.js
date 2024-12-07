use std::ops::{Deref, DerefMut};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Stack {
    pub(crate) ptr: *mut nu_protocol::engine::Stack,
}

#[wasm_bindgen]
impl Stack {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Stack {
        let ptr = Box::new(nu_protocol::engine::Stack::new());
        Stack {
            ptr: Box::into_raw(ptr),
        }
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.ptr)) }
    }
}

impl Deref for Stack {
    type Target = nu_protocol::engine::Stack;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}
