import { allocEngineState, freeEngineState } from "../wasm/nu_js_core.wasm";
import { ptr } from "./utils/ptr.js";

export class EngineState {
  /**
   * @type {number}
   */
  [ptr];
  
  constructor() {
    this[ptr] = allocEngineState();
    EngineState.finalizer.register(this, this[ptr]);
  }

  /**
   * @type {FinalizationRegistry<number>}
   * @static
   * @private
   */
  static finalizer = new FinalizationRegistry((ptr) => {
    freeEngineState(ptr);
  });
}
