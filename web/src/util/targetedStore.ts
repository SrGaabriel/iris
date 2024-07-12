export default class TargetedStore<T> {
    // Map of String -> callback
    private readonly store: Record<string, (value: T) => void> = {};

    constructor() {
        this.store = {};
    }

    subscribe(key: string, callback: (value: T) => void) {
        this.store[key] = callback;
    }

    dispatch(key: string, value: T) {
        if (this.store[key]) {
            this.store[key](value);
        }
    }

    unsubscribe(key: string) {
        delete this.store[key];
    }
}