import {
  EngineState,
  execute,
  Stack,
  TryFromValueError,
  UnsupportedValueError,
  ParseError,
  CompileError,
} from "../pkg/deno/nu_js.js";

const engineState = new EngineState();
const stack = new Stack();

try {
  execute("", engineState, stack, {
    input: { type: "bool", value: "lol" } as any,
  });
} catch (e) {
  if (e instanceof TryFromValueError) console.log("TryFromValueError catched");
  else console.log("TryFromValueError not catched");
}

try {
  execute("", engineState, stack, {
    input: { type: "unsupported" },
  });
} catch (e) {
  if (e instanceof UnsupportedValueError) {
    console.log("UnsupportedValueError catched");
  } else console.log("UnsupportedValueError not catched");
}

try {
  execute("for item in {", engineState, stack, {});
} catch (e) {
  if (e instanceof ParseError) {
    console.log("ParseError catched");
    console.log(e.message);
  } else console.log("ParseError not catched");
}

try {
  execute("lol", engineState, stack, {});
} catch (e) {
  if (e instanceof CompileError) {
    console.log("CompileError catched");
    console.log(e.message);
  } else console.log("CompileError not catched");
}
