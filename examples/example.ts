import { EngineState, Stack, TryFromValueError, UnsupportedValueError, execute} from "../pkg/deno/nu_js.js";

const engineState = new EngineState();
const stack = new Stack();

try {
  execute("", engineState, stack, {
    input: {type: "bool", value: "lol"} as any,
  });
}
catch (e) {
  if (e instanceof TryFromValueError) console.log("TryFromValueError catched");
  else console.log("TryFromValueError not catched");
}

try {
  execute("", engineState, stack, {
    input: {type: "unsupported"},
  });
}
catch (e) {
  if (e instanceof UnsupportedValueError) console.log("UnsupportedValueError catched");
  else console.log("UnsupportedValueError not catched");
}

