import { EngineState, Stack, Value, yeet, TryFromValueError} from "../pkg/deno/nu_js.js";

// let engineState = new EngineState();
// console.log({
//   engineState
// });

const error = new TryFromValueError("some message", {});
console.log(error);
console.log(error instanceof TryFromValueError);
console.log(error instanceof Error);

console.log(yeet() instanceof TryFromValueError);

// console.log(new EngineState());
// console.log(new Stack());
// console.log(Value.some_string().toString());
// console.log(Value.some_string().toJSON());
