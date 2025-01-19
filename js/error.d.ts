export declare class TryFromValueError extends Error {
    readonly value: object;
    constructor(message: string, value: object);
}
