import {execute, EngineState, Stack} from "nu.js/nodejs";

let engineState = new EngineState();
let stack = new Stack();
let version = execute("version", engineState, stack, {});
if (version.type != "record") throw new Error("expected a record");
console.log(version.value);
