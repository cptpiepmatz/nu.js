export class NuJsError extends Error {}

export class TryFromValueError extends NuJsError {
  constructor(message: string, public readonly value: object) {
    super(message);
  }
}

export class UnsupportedValueError extends NuJsError {}
