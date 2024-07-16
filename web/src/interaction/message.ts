export let Packet: any;
export const CLIENTBOUND_ID_TO_PROTOBUF_OBJECT: { [key: number]: any } = {};
export const SERVERBOUND_ID_TO_PROTOBUF_OBJECT: { [key: number]: any } = {};

// SERVERBOUND
export const CONTEXT_READ_ID = 1;
export const TYPING_REQUEST_ID = 2;

// CLIENTBOUND

export const TEXT_MESSAGE_ID = 2;
export const MESSAGES_READ_ID = 3;
export const CONTACT_TYPING_ID = 4;
export const MESSAGE_EDITED_ID = 5;
export const MESSAGE_DELETED_ID = 6;

export function loadProto() {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    window.protobuf.load("messages/Packet.proto", function(err, root) {
        if (err) {
            console.log("Error loading proto file Packet");
            throw err;
        }

        Packet = root.lookupType("Packet");
    });
    // SERVERBOUND
    loadProtoFile(CONTEXT_READ_ID, "ContextRead", false);
    loadProtoFile(TYPING_REQUEST_ID, "TypingRequest", false);

    // CLIENTBOUND
    loadProtoFile(TEXT_MESSAGE_ID, "TextMessage", true);
    loadProtoFile(MESSAGES_READ_ID, "MessagesRead", true);
    loadProtoFile(CONTACT_TYPING_ID, "ContactTyping", true);
    loadProtoFile(MESSAGE_EDITED_ID, "MessageEdited", true);
    loadProtoFile(MESSAGE_DELETED_ID, "MessageDeleted", true);
}

function loadProtoFile(id: number, name: string, clientbound: boolean): any {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    window.protobuf.load(`messages/${name}.proto`, function(err, root) {
        if (err) {
            console.log("Error loading proto file " + name);
            throw err;
        }
        const ID_TO_PROTOBUF_OBJECT = clientbound ? CLIENTBOUND_ID_TO_PROTOBUF_OBJECT : SERVERBOUND_ID_TO_PROTOBUF_OBJECT;
        ID_TO_PROTOBUF_OBJECT[id] = root.lookupType(name);
    });
}

export function encodePacket(id: number, packet: object) {
    const object = SERVERBOUND_ID_TO_PROTOBUF_OBJECT[id];
    const message = {
        id: id,
        data: object.encode(packet).finish()
    };
    return Packet.encode(message).finish();
}

function encodePacketData(packet: any) {
    const object = SERVERBOUND_ID_TO_PROTOBUF_OBJECT[packet.id];
    if (object) {
        return object.encode(packet).finish();
    } else {
        console.log("Unknown packet id: " + packet.id);
    }
}

export function decodePacket(packet: any) {
    const object = CLIENTBOUND_ID_TO_PROTOBUF_OBJECT[packet.id];
    if (object) {
        return object.decode(packet.data);
    } else {
        console.log("Unknown packet id: " + packet.id);
    }
}

