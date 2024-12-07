import * as wasm from "./nu_js_bg.wasm";
export * from "./nu_js_bg.js";
import { __wbg_set_wasm } from "./nu_js_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
