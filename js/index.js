import {stuff} from "../wasm/nu_js_core.wasm";

export class EngineState {
  pointer;
  
  constructor() {
    this.pointer = 0;
    console.log(stuff());
  }
}
