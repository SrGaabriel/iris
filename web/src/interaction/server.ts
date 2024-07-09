import {writable} from "svelte/store";
import {browser} from "$app/environment";
import {TextMessage} from "./message.ts";

let socket: WebSocket;

if (browser) {
    socket = new WebSocket('wss://56d6-179-130-188-11.ngrok-free.app/ws');

    socket.addEventListener('open', () => {
        console.log('Connected to server');
    });

    socket.addEventListener('message', (event) => {
        event.data.arrayBuffer().then(buffer => {
            const message = TextMessage.decode(new Uint8Array(buffer));
            messageStore.set(message.content);
        });
    });
}

const messageStore = writable('');

const sendMessage = (message: string) => {
    socket.send(TextMessage.encode({
        content: message
    }).finish());
}

export default {
    subscribe: messageStore.subscribe,
    sendMessage
}