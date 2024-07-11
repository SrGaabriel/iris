import {writable} from "svelte/store";
import {browser} from "$app/environment";
import {loadProto, TextMessage} from "./message.ts";

export const SECURE = false;
export const PROTOCOL = SECURE ? "s" : "";
export const DOMAIN = 'localhost:3000';
export const API = `http${PROTOCOL}://${DOMAIN}`;
export const WEBSOCKET = `ws${PROTOCOL}://${DOMAIN}`;

let socket: WebSocket;

function connect(token: string) {
    if (browser) {
        loadProto();
        socket = new WebSocket(`${WEBSOCKET}/ws`, ['Token', token]);

        socket.addEventListener('open', () => {
            console.log('Connected to server');
        });

        socket.addEventListener('message', (event) => {
            event.data.arrayBuffer().then((buffer: Iterable<number>) => {
                const message = TextMessage.decode(new Uint8Array(buffer));
                messageStore.set(message);
            });
        });
    }
}

const messageStore = writable();

const sendMessage = (message: string) => {
    socket.send(TextMessage.encode({
        content: message
    }).finish());
}

function subscribeToContext(context: bigint, callback: (message: any) => void) {
    return messageStore.subscribe((message) => {
        if (message.context === context) {
            callback(message);
        }
    });
}

export default {
    subscribe: messageStore.subscribe,
    subscribeToContext,
    sendMessage,
    connect
}