export let Packet: any;
export const ID_TO_PROTOBUF_OBJECT: { [key: number]: any } = {};

// SERVERBOUND
export const CONTEXT_READ_ID = 1;

// CLIENTBOUND

export const TEXT_MESSAGE_ID = 2;
export const MESSAGES_READ_ID = 3;

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
    loadProtoFile(CONTEXT_READ_ID, "ContextRead");
    loadProtoFile(TEXT_MESSAGE_ID, "TextMessage");
    loadProtoFile(MESSAGES_READ_ID, "MessagesRead");
}

function loadProtoFile(id: number, name: string): any {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    window.protobuf.load(`messages/${name}.proto`, function(err, root) {
        if (err) {
            console.log("Error loading proto file " + name);
            throw err;
        }
        ID_TO_PROTOBUF_OBJECT[id] = root.lookupType(name);
    });
}

export function encodePacket(id: number, packet: object) {
    const object = ID_TO_PROTOBUF_OBJECT[id];
    const message = {
        id: id,
        data: object.encode(packet).finish()
    };
    return Packet.encode(message).finish();
}

function encodePacketData(packet: any) {
    const object = ID_TO_PROTOBUF_OBJECT[packet.id];
    if (object) {
        return object.encode(packet).finish();
    } else {
        console.log("Unknown packet id: " + packet.id);
    }
}

export function decodePacket(packet: any) {
    const object = ID_TO_PROTOBUF_OBJECT[packet.id];
    if (object) {
        return object.decode(packet.data);
    } else {
        console.log("Unknown packet id: " + packet.id);
    }
}

