use nu_protocol::engine::{EngineState, Stack};

#[no_mangle]
pub extern "C" fn allocEngineState() -> *mut EngineState {
    let engine_state = Box::new(EngineState::new());
    Box::into_raw(engine_state)
}

#[no_mangle]
pub extern "C" fn freeEngineState(ptr: *mut EngineState) {
    unsafe { drop(Box::from_raw(ptr)) }
}

#[no_mangle]
pub extern "C" fn allocStack() -> *mut Stack {
    let stack = Box::new(Stack::new());
    Box::into_raw(stack)
}

#[no_mangle]
pub extern "C" fn freeStack(ptr: *mut Stack) {
    unsafe { drop(Box::from_raw(ptr)) }
}

#[no_mangle]
pub extern "C" fn execute() {

}
