import {allocStack, freeStack} from "../wasm/nu_js_core.wasm";
import { ptr } from "./utils/ptr.js";

export class Stack {
  /**
   * @type {number}
   */
  [ptr];
  
  constructor() {
    this[ptr] = allocStack();
    Stack.finalizer.register(this, this[ptr]);
  }

  /**
   * @type {FinalizationRegistry<number>}
   * @static
   * @private
   */
  static finalizer = new FinalizationRegistry((ptr) => {
    freeStack(ptr);
  });
}
