export class NuJsError extends Error {}

export class TryFromValueError extends NuJsError {
  constructor(message: string, public readonly value: object) {
    super(message);
  }
}

export class UnsupportedValueError extends NuJsError {}
export class ParseError extends NuJsError {}
export class CompileError extends NuJsError {}
export class MergeDeltaError extends NuJsError {}
export class EvalError extends NuJsError {}
export class CollectResultsError extends NuJsError {}
