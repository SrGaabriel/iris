import {writable} from "svelte/store";
import {browser} from "$app/environment";
import {loadProto, TextMessage} from "./message.ts";

export const DOMAIN = 'localhost:7989';

let socket: WebSocket;

function connect() {
    if (browser) {
        loadProto();
        socket = new WebSocket(`ws://${DOMAIN}/ws`);

        socket.addEventListener('open', () => {
            console.log('Connected to server');
        });

        socket.addEventListener('message', (event) => {
            event.data.arrayBuffer().then((buffer: Iterable<number>) => {
                const message = TextMessage.decode(new Uint8Array(buffer));
                messageStore.set(message.content);
            });
        });
    }
}

const messageStore = writable('');

const sendMessage = (message: string) => {
    socket.send(TextMessage.encode({
        content: message
    }).finish());
}

export default {
    subscribe: messageStore.subscribe,
    sendMessage,
    connect
}