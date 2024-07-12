<script lang="ts">
    import {onMount} from "svelte";
    import {API} from "../../interaction/server.ts";

    export let token: string;
    export let user: any;
    export let contact: any;
    export let store: any;

    let messages: any[] = [];
    let typingMessage = '';
    let messagesElement: any;

    onMount(() => {
        messagesElement = document.querySelector('.messages');
        store.subscribe((message) => {
            console.log("at last")
            if (!message) return;
            if (message.context === user.id) {
                const newMessage = {
                    ...message, user_id: message.userId
                }
                messages = [...messages, newMessage];
                messagesElement.scrollTo(0, messagesElement.scrollHeight);
            }
        });
        fetch(`${API}/api/messages/${contact.id}`, {
            method: 'GET',
            headers: {
                'Authorization': 'Bearer ' + token
            }
        }).then((request) => request.json()).then((messageList) => {
            messages = messageList;
        }).then(() => {
            messagesElement.scrollTo(0, messagesElement.scrollHeight);
        }).catch((error) => {
            console.error(error);
        });
    })

    function submit() {
        messagesElement.scrollTo(0, messagesElement.scrollHeight);
        fetch(`${API}/api/messages/${contact.id}`, {
            method: 'POST',
            headers: {
                'Authorization': 'Bearer ' + token,
            },
            body: JSON.stringify({
                content: typingMessage
            })
        }).then(response => response.text()).then((text) => JSONbig.parse(text)).then((message) => {
            messages = [...messages, message];
            typingMessage = '';
        }).then(() => {
            messagesElement.scrollTo(0, messagesElement.scrollHeight);
        }).catch((error) => {
            console.error(error);
        });
    }

    function getUserObject(id: bigint): any {
        if (id.toString() === user.id.toString()) {
            return user;
        } else {
            return contact;
        }
    }
</script>

<div class="chat">
    <div class="header">
        <span class="contact-name">{contact?.name}</span>
    </div>
    <div class="messages">
        <div class="messages-container">
            {#each messages as message}
                {@const sender = getUserObject(message.user_id)}
                {@const sent = sender.id === user.id}
                <div class="message-container {sent ? 'sent' : 'received'}">
                    <div class="message-sender">
                        <span class="message-sender-name">{sender.name}</span>
                    </div>
                    <div class="message-text-container">
                        <span class={`message ${sent ? 'sent' : 'received'}`}>{message.content}</span>
                    </div>
                </div>
            {/each}
        </div>
        <div class="footer-space"></div>
    </div>
    <div class="send-container">
        <div class="send-input-container">
            <input class="send-input" type="text" bind:value={typingMessage} placeholder="Type your message..." />
        </div>
        <button class="send-button" on:click={submit}>Send</button>
    </div>
</div>

<style>
    .chat {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 95%;
        height: 95%;
        background-color: #2a2a2a;
        border-radius: 16px;
    }
    .header {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
        height: 75px;
    }
    .contact-name {
        color: white;
        font-family: 'DM Sans', sans-serif;
        font-size: 32px;
        font-weight: 600;
    }
    .messages {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
        height: 100%;
        overflow-y: scroll;
    }
    .messages-container {
        display: flex;
        flex-direction: column;
        width: 95%;
        margin-bottom: 40px;
    }
    .message-container {
        display: flex;
        flex-direction: column;
        margin: 12px 48px;
        gap: 4px;
    }
    .message-container.sent {
        align-items: flex-end;
    }
    .message-container.received {
        align-items: flex-start;
    }
    .message {
        color: #ebebeb;
        display: inline-block;
        padding: 10px 18px;
        border-radius: 64px;
        width: auto;
        max-width: 400px;
        font-family: 'DM Sans', sans-serif;
        font-size: 17px;
        text-wrap: auto;
        word-wrap: normal;
        overflow-wrap: normal;
        word-break: break-word;
    }
    .message.sent {
        background-color: #6451b8;
    }
    .message.received {
        background-color: #181818;
    }
    .message-sender-name {
        font-family: 'DM Sans', sans-serif;
        color: #c9c9c9;
        font-size: 14px;
        margin-left: 2px;
    }
    .send-container {
        display: flex;
        align-items: center;
        width: 95%;
        height: 60px;
        border-radius: 8px;
        padding: 4px;
        background-color: #111111;
        margin: 24px;
    }
    .send-input-container {
        background-color: #202020;
        width: 90%;
        height: 36px;
        margin: 0 16px;
        border-radius: 12px;
    }
    .send-input {
        outline: none;
        border: none;
        width: 100%;
        color: white;
        background-color: transparent;
        font-family: 'DM Sans', sans-serif;
        font-size: 15px;
        padding: 8px 16px;
        border-radius: 24px;
    }
    .send-button {
        font-size: 18px;
        font-weight: 500;
        color: #2a2a2a;
        border-radius: 12px;
        font-family: 'DM Sans', sans-serif;
        background-color: white;
        margin-left: auto;
        border: none;
        width: 80px;
        cursor: pointer;
        height: 90%;
        margin-right: 12px;
    }
    .send-button:hover {
        background-color: #4f4fc7;
    }
</style>