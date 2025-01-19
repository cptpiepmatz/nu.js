export class TryFromValueError extends Error {
  constructor(message: string, public readonly value: object) {
    super(message);
  }
}
