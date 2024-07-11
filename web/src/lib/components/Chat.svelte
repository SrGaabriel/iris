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
        <h1>{contact?.name}</h1>
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
    </div>
    <input type="text" placeholder="Type a message..." bind:value={typingMessage}/>
    <button on:click={() => submit()}>Send</button>
</div>

<style>
    .chat {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
    }
    .header {
        width: 100%;
        height: 75px;
        background-color: white;
        border-bottom: 2px solid #e9e9e9;
    }
    .messages {
        display: flex;
        justify-content: center;
        width: 100%;
        height: 100%;
        box-shadow: 4px 4px 6px black;
        overflow: scroll;
    }
    .messages-container {
        display: flex;
        flex-direction: column;
        width: 75%;
        margin-bottom: 20px;
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
        color: #2b2b2b;
        display: inline-block;
        padding: 10px 18px;
        border-radius: 64px;
        width: auto;
        max-width: 400px;
        font-family: 'DM Sans', sans-serif;
        font-size: 17px;
        text-wrap: normal;
        word-wrap: normal;
        overflow-wrap: normal;
        word-break: break-word;
    }
    .message.sent {
        background-color: #50aed2;
    }
    .message.received {
        background-color: #bcbcbc;
    }
    .message-sender-name {
        font-family: 'DM Sans', sans-serif;
        font-size: 14px;
        margin-left: 2px;
    }
</style>