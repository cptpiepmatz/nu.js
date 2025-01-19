export class TryFromValueError extends Error {
    value;
    constructor(message, value) {
        super(message);
        this.value = value;
    }
}
