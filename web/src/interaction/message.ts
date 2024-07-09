import {browser} from "$app/environment";

export let TextMessage: any;

if (browser) {
    window.protobuf.load("messages/TextMessage.proto", function(err, root) {
       if (err) {
           console.log("Error loading proto file");
           throw err;
       }

       TextMessage = root.lookupType("TextMessage");
    });
}

