export class CustomTargetedStore<K extends string | number | symbol, V> {
    // Map of String -> callback
    private readonly store: Record<K, ((value: V) => void)[]> = {};
    private readonly all: ((key: K, value: V) => void)[] = [];

    subscribe(key: K, callback: (value: V) => void) {
        if (!this.store[key]) {
            this.store[key] = [callback];
        }
        this.store[key].push(callback);
    }

    subscribeAll(callback: (key: K, value: V) => void) {
        this.all.push(callback);
    }

    dispatch(key: K, value: V) {
        if (this.store[key]) {
            this.store[key].forEach((callback) => {
                callback(value);
            });
        }
        this.all.forEach((callback) => {
            callback(key, value);
        });
    }
}

export default class TargetedStore<T> {
    // Map of String -> callback
    private readonly store: Record<string, (value: T) => void> = {};
    private readonly all: ((key: string, value: T) => void)[] = [];

    constructor() {
        this.store = {};
    }

    subscribe(key: string, callback: (value: T) => void) {
        this.store[key] = callback;
    }

    subscribeAll(callback: (key: string, value: T) => void) {
        this.all.push(callback);
    }

    dispatch(key: string, value: T) {
        if (this.store[key]) {
            this.store[key](value);
        }
        this.all.forEach((callback) => {
            callback(key, value);
        });
    }

    unsubscribe(key: string) {
        delete this.store[key];
    }
}