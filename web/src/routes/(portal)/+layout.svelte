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
        --chat-receiver-color: #ffffff;
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
        --chat-receiver-color: #181818;
    }
</style>