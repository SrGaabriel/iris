<slot></slot>

<style>
    :global(body.lightmode) {
        --text-color: #0a0a0a;
        --light-contrast: #ebebeb;
        --heavy-constrast: #d9d9d9;
        --background: #ffffff;
        --thin-border: #dedede;
        --biography-contact: #6a6a6a;
        --delicate-hover-contrast: 0, 0, 0;
        --blurred-username: #b1b1b1;
        --selected-contact: #cceaf0;
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
        --light-contrast: #2a2a2a;
        --heavy-constrast: #202020;
        --background: #141414;
        --thin-border: #646464;
        --biography-contact: #929292;
        --delicate-hover-contrast: 255, 255, 255;
        --blurred-username: #626262;
        --selected-contact: #222427;
        --theme-toggle-contrast: #ffffff;
        --message-sender-name: #c9c9c9;
        --chat-sender-color: #6451b8;
        --chat-sender-color-contrast: #9181b4;
        --chat-sender-color-contrast-hover: #7d71a2;
        --chat-sender-color-dark-contrast: #31246f;
        --chat-receiver-color: #181818;
        --chat-receiver-color-contrast: #55555b;
        --chat-receiver-color-contrast-hover: #404040;
        --chat-receiver-color-dark-contrast: #d1d1d1;
        --message-details: #929292;
        --reply-color: #e0e0e0;
        --reply-color-content: #c9c9c9;
        --reply-color-details: #b5b5b5;
        --reaction-counter: #c9c9c9;
        --reaction-selected-border: #c20af4;
        --reaction-selected-background: #841c96;
        --input-add-emoji-hover: #a81fb2;
    }
</style>