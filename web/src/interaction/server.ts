import {writable} from "svelte/store";
import {browser} from "$app/environment";
import {decodePacket, encodePacket, loadProto, Packet} from "./message.ts";
import TargetedStore, {CustomTargetedStore} from "../util/targetedStore.ts";

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
                const packet = Packet.decode(new Uint8Array(buffer));
                const message = decodePacket(packet);
                messageStore.dispatch(packet.id, message);
            });
        });
    }
}

const messageStore = new CustomTargetedStore<number, any>();

const sendPacket = (id: number, packet: object) => {
    if (socket.readyState === WebSocket.OPEN) {
        const encoded = encodePacket(id, packet);
        socket.send(encoded);
    }
}

export default {
    store: messageStore,
    sendPacket,
    connect
}