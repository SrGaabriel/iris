<slot></slot>

<style>
    :global(body.lightmode) {
        --text-color: #0a0a0a;
        --icon-color: #525561;
        --icon-selected: rgb(206, 206, 207);
        --icon-selected-contrast: #2d3c4d;
        --light-contrast: #ebebeb;
        --light-heavier-contrast: #d4d4d4;
        --heavy-constrast: #d9d9d9;
        --background: #ffffff;
        --thin-border: #b2b2b2;
        --biography-contact: #6a6a6a;
        --delicate-hover-contrast: 0, 0, 0;
        --blurred-username: #b1b1b1;
        --selected-contact: #e7e7e7;
        --theme-toggle-contrast: #262626;
        --message-sender-name: #6a6a6a;
        --chat-sender-color: #89dceb;
        --chat-sender-color-contrast: #cee4e9;
        --chat-sender-color-contrast-hover: #c0dbe0;
        --chat-sender-color-dark-contrast: #53a2b2;
        --chat-receiver-color-contrast: #dedddd;
        --chat-receiver-color-contrast-hover: #d0d0d0;
        --chat-receiver-color-dark-contrast: #565656;
        --chat-receiver-color: #ffffff;
        --message-details: #9c9c9c;
        --reply-to: #5b5b5b;
        --reply-color: #3f3f3f;
        --reply-color-content: #6e6e6e;
        --reply-color-details: #8d8888;
        --reaction-counter: #525252;
        --reaction-selected-border: #2385be;
        --reaction-selected-background: #9fecf4;
        --input-add-emoji-hover: #fff100;
    }
    :global(body:not(.lightmode)) {
        --text-color: #fcfcfc;
        --icon-color: #7f8497;
        --icon-selected: #44474a;
        --icon-selected-contrast: #9fa6bc;
        --light-contrast: #2a2a2a;
        --light-heavier-contrast: #262626;
        --heavy-constrast: #202020;
        --background: #141414;
        --thin-border: #646464;
        --biography-contact: #929292;
        --delicate-hover-contrast: 255, 255, 255;
        --blurred-username: #626262;
        --selected-contact: #222427;
        --theme-toggle-contrast: #ffffff;
        --message-sender-name: #c9c9c9;
        --chat-sender-color: #5175b8;
        --chat-sender-color-contrast: #818ab4;
        --chat-sender-color-contrast-hover: #718aa2;
        --chat-sender-color-dark-contrast: #31246f;
        --chat-receiver-color: #181818;
        --chat-receiver-color-contrast: #55555b;
        --chat-receiver-color-contrast-hover: #404040;
        --chat-receiver-color-dark-contrast: #d1d1d1;
        --message-details: #929292;
        --reply-to: #a09f9f;
        --reply-color: #e0e0e0;
        --reply-color-content: #c9c9c9;
        --reply-color-details: #b5b5b5;
        --reaction-counter: #c9c9c9;
        --reaction-selected-border: #c20af4;
        --reaction-selected-background: #841c96;
        --input-add-emoji-hover: #a81fb2;
    }
</style>