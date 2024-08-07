export let Packet: any;
export const CLIENTBOUND_ID_TO_PROTOBUF_OBJECT: { [key: number]: any } = {};
export const SERVERBOUND_ID_TO_PROTOBUF_OBJECT: { [key: number]: any } = {};

// SERVERBOUND
export const CHANNEL_READ_ID = 1;
export const TYPING_REQUEST_ID = 2;

// CLIENTBOUND

export const MESSAGE_CREATED_ID = 2;
export const MESSAGES_READ_ID = 3;
export const CONTACT_TYPING_ID = 4;
export const MESSAGE_EDITED_ID = 5;
export const MESSAGE_DELETED_ID = 6;
export const REACTION_ADDED_ID = 7;
export const REACTION_REMOVED_ID = 8;

export function loadProto() {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    window.protobuf.load("/messages/Packet.proto", function(err, root) {
        if (err) {
            console.log("Error loading proto file Packet");
            throw err;
        }

        Packet = root.lookupType("Packet");
    });
    // // SERVERBOUND
    // loadProtoFile(CHANNEL_READ_ID, "ChannelRead", false);
    // loadProtoFile(TYPING_REQUEST_ID, "TypingRequest", false);
    //
    // // CLIENTBOUND
    // loadProtoFile(MESSAGE_CREATED_ID, "MessageCreated", true);
    // loadProtoFile(MESSAGES_READ_ID, "MessagesRead", true);
    // loadProtoFile(CONTACT_TYPING_ID, "ContactTyping", true);
    // loadProtoFile(MESSAGE_EDITED_ID, "MessageEdited", true);
    // loadProtoFile(MESSAGE_DELETED_ID, "MessageDeleted", true);
    // loadProtoFile(REACTION_ADDED_ID, "ReactionAdded", true);
    // loadProtoFile(REACTION_REMOVED_ID, "ReactionRemoved", true);
}

function loadProtoFile(id: number, name: string, clientbound: boolean): any {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    window.protobuf.load(`/messages/${name}.proto`, function(err, root) {
        if (err) {
            console.log("Error loading proto file " + name);
            throw err;
        }
        const ID_TO_PROTOBUF_OBJECT = clientbound ? CLIENTBOUND_ID_TO_PROTOBUF_OBJECT : SERVERBOUND_ID_TO_PROTOBUF_OBJECT;
        ID_TO_PROTOBUF_OBJECT[id] = root.lookupType(name);
    });
}

export function encodePacket(id: number, packet: object) {
    const message = {
        id: id,
        data: encodePacketData(packet)
    };
    return Packet.encode(message).finish();
}

function encodePacketData(packet: any) {
    // bytes of json
    return new TextEncoder().encode(JSON.stringify(packet));
}

export function decodePacket(packet: any) {
    return JSON.parse(new TextDecoder().decode(packet.data));
}

