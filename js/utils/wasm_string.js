export class WasmString {
  /**
   * @type {WebAssembly.Memory}
   */
  memory;

  constructor(string) {
    const encoder = new TextEncoder();
    const bytes = encoder.encode(string);
    
  }
}
