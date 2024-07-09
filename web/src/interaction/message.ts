export let TextMessage: any;

export function loadProto() {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    window.protobuf.load("messages/TextMessage.proto", function(err, root) {
       if (err) {
           console.log("Error loading proto file");
           throw err;
       }

       TextMessage = root.lookupType("TextMessage");
    });
}

