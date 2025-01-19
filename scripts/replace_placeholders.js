import process from "node:process";
import fs from "node:fs";

let path, replacer, replacement;

const esmReplacement = "export { TryFromValueError, UnsupportedValueError };";

const arg = process.argv[2]
switch (arg) {
  case undefined:
    throw new Error("missing first argument");
  case "bundler":
    path = "pkg/bundler/nu_js_bg.js";
    replacer = `
      export function __nu_js__error__placeholder() {
          wasm.__nu_js__error__placeholder();
      }
    `;
    replacement = esmReplacement;
    break;
  case "deno":
    path = "pkg/deno/nu_js.js";
    replacer = `
      export function __nu_js__error__placeholder() {
          wasm.__nu_js__error__placeholder();
      }
    `;
    replacement = esmReplacement;
    break;
  case "nodejs":
    path = "pkg/nodejs/nu_js.js";
    replacer = `
      module.exports.__nu_js__error__placeholder = function() {
          wasm.__nu_js__error__placeholder();
      };
    `;
    replacement = [
      "module.exports.TryFromValueError = TryFromValueError;",
      "module.exports.UnsupportedValueError = UnsupportedValueError;"
    ].join("\n");
    break;
  case "web":
    path = "pkg/web/nu_js.js";
    replacer = `
      export function __nu_js__error__placeholder() {
          wasm.__nu_js__error__placeholder();
      }
    `;
    replacement = esmReplacement;
    break;
  default:
    throw new Error(`got "${arg}", expected "bundler", "deno", "nodejs" or "deno"`);
}

replacer = replacer.trim().replace(/^ {6}/gm, "");

let content = fs.readFileSync(path, "utf-8");
const replacerIndex = content.indexOf(replacer);
if (replacerIndex == -1) {
  throw new Error(`replacer not found in "${path}"`);
}

content = content.replace(replacer, replacement);
fs.writeFileSync(path, content);
