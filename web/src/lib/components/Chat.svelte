<script lang="ts">
    import {onMount} from "svelte";
    import {API} from "../../interaction/server.ts";
    import type {User} from "$lib/user.ts";
    import {countEmojis, isMessageMadeOfOnlyEmojis} from "$lib/util/emojis.ts";
    import type TargetedStore from "../../util/targetedStore.ts";
    import {getTimestamp, getTimestampFormatted} from "$lib/util/snowflake.ts";

    export let token: string;
    export let user: User;
    export let contact: User;
    export let store: any;
    export let keyStore: TargetedStore;

    let messages: any[] = [];
    let typingMessage = '';

    let messagesElement: any;
    let inputElement: any;

    onMount(() => {
        messagesElement = document.getElementById('messages');
        inputElement = document.getElementById('send-input')! as HTMLTextAreaElement;

        function recalculateHeight() {
            let linebreaks = (inputElement.value.match(/\n/g) || []).length;
            inputElement.style.height = Math.min(24+linebreaks*12, 40) + 'px';
        }

        inputElement.addEventListener('input', () => {
            recalculateHeight()
        });

        keyStore.subscribe(contact.id.toString(), (key) => {
            if (!key) return;
            if (key === 'Enter') {
                submit();
                return;
            }
            if (document.activeElement !== inputElement) {
                inputElement.focus();
            }
        });

        store.subscribe((message) => {
            if (!message) return;
            if (message.userId === contact.id) {
                const newMessage = {
                    ...message, user_id: message.userId
                }
                messages = [...messages, newMessage];
                setTimeout(() => {
                    messagesElement.scrollTo(0, messagesElement.scrollHeight);
                }, 50);
            }
        });
        fetch(`${API}/api/messages/${contact.id}`, {
            method: 'GET',
            headers: {
                'Authorization': 'Bearer ' + token
            }
        }).then((request) => request.json()).then((messageList) => {
            messages = messageList.reverse();
        }).then(() => {
            messagesElement.scrollTo(0, messagesElement.scrollHeight);
        }).catch((error) => {
            console.error(error);
        });
    })

    function submit() {
        if (!typingMessage || typingMessage.trim().length === 0) return;
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
            inputElement.style.height='24px';
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

    function formatReceipt(receipt: string): string {
        switch (receipt) {
            case 0:
                return 'Sent';
            case 1:
                return 'Delivered';
            case 2:
                return 'Read';
        }
    }
</script>

<div class="chat" id="chat-{contact.id}">
    <div class="header">
        <span class="contact-name">{contact?.name}</span>
    </div>
    <div class="messages" id="messages">
        <div class="messages-container" data-relevant="true">
            {#each messages as message,i}
                {@const sender = getUserObject(message.user_id)}
                {@const sent = sender.id === user.id}
                <div class="message-container {sent ? 'sent' : 'received'}">
                    <div class="message-sender">
                        <span class="message-sender-name">{sender.name}</span>
                    </div>
                    <div class="message-text-container">
                        <span
                                class={`message ${sent ? 'sent' : 'received'}`}
                                data-only-emojis={isMessageMadeOfOnlyEmojis(message.content)}
                                data-emoji-count={countEmojis(message.content)}
                        >{@html message.content.replace(/\n/g, '<br>')}</span>
                    </div>
                    <div class="message-details">
                        {#if !messages[i+1] || messages[i+1].user_id !== message.user_id}
                            <span>{getTimestampFormatted(getTimestamp(message.id))}</span>
                            {#if message.user_id === user.id}
                                •
                                <span>{formatReceipt(message.receipt)}</span>
                            {/if}
                        {/if}
                    </div>
                </div>
            {/each}
        </div>
        <div class="footer-space"></div>
    </div>
    <div class="send-container">
        <div class="send-input-container">
            <textarea
                    id="send-input"
                    class="send-input"
                    rows="1"
                    bind:value={typingMessage}
                    placeholder="Type your message..."
            />
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
        background-color: var(--light-contrast);
        border-radius: 12px;
    }
    .header {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
        height: 75px;
    }
    .contact-name {
        color: var(--text-color);
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
        border-radius: inherit;
        margin: 12px 48px;
        gap: 6px;
    }
    .message-container.sent {
        align-items: flex-end;
    }
    .message-container.received {
        align-items: flex-start;
    }
    .message {
        color: var(--text-color);
        display: inline-block;
        padding: 10px 18px;
        border-radius: 12px;
        width: auto;
        max-width: 400px;
        font-family: 'DM Sans', sans-serif;
        font-size: 17px;
        text-wrap: auto;
        word-wrap: normal;
        overflow-wrap: normal;
        word-break: break-word;
    }
    .message[data-only-emojis="true"] {
        font-size: 32px;
    }
    .message-text-container {
        border-radius: inherit;
    }
    .message.sent {
        background-color: var(--chat-sender-color);
    }
    .message.received {
        background-color: var(--chat-receiver-color);
    }
    .message-sender-name {
        font-family: 'DM Sans', sans-serif;
        color: var(--message-sender-name);
        font-size: 14px;
        margin-left: 2px;
        font-weight: 500;
    }
    .message-details {
        font-family: 'DM Sans', sans-serif;
        font-size: 12px;
    }
    .send-container {
        display: flex;
        align-items: center;
        width: 95%;
        min-height: 60px;
        border-radius: 8px;
        padding: 4px;
        background-color: var(--background);
        margin: 24px;
    }
    .send-input-container {
        background-color: var(--light-contrast);
        width: 92%;
        min-height: 36px;
        margin: 0 10px;
        border-radius: 12px;
    }
    .send-input {
        display: flex;
        align-items: center;
        justify-content: center;
        outline: none;
        border: none;
        width: 100%;
        height: 24px;
        color: var(--text-color);
        background-color: transparent;
        font-family: 'DM Sans', sans-serif;
        font-size: 15px;
        padding: 8px 16px;
        border-radius: 12px;
        text-wrap: wrap;
        word-wrap: normal;
        overflow-wrap: normal;
        word-break: break-word;
        resize: none;
    }
    .send-button {
        font-size: 18px;
        font-weight: 600;
        color: var(--background);
        border-radius: 6px;
        font-family: 'DM Sans', sans-serif;
        background-color: var(--text-color);
        margin-left: auto;
        border: none;
        width: 80px;
        cursor: pointer;
        height: 90%;
        margin-right: 12px;
        transition: background-color 0.1s;
    }
    .send-button:hover {
        background-color: #e3e3e3;
    }
</style>