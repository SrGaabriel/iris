<script lang="ts">
    import {onMount} from "svelte";
    import server, {API} from "../../interaction/server.ts";
    import type {User} from "$lib/user.ts";
    import {countEmojis, isMessageMadeOfOnlyEmojis} from "$lib/util/emojis.ts";
    import type TargetedStore from "../../util/targetedStore.ts";
    import {getTimestamp, getTimestampFormatted} from "$lib/util/snowflake.ts";
    import {quintOut} from "svelte/easing";
    import {
        MESSAGE_DELETED_ID,
        MESSAGE_EDITED_ID,
        MESSAGES_READ_ID,
        TYPING_REQUEST_ID
    } from "../../interaction/message.ts";
    import {TYPING_DELAY} from "$lib/constants.ts";
    import {crossfade} from "svelte/transition";
    import Alert from "$lib/components/Alert.svelte";
    import {writable} from "svelte/store";

    export let token: string;
    export let user: User;
    export let contact: User;
    export let store: any;
    export let keyboardEventStore: TargetedStore;
    export let typing: boolean;
    export let isSelfTyping: boolean;

    let messages: any[] = [];
    let typingMessage = '';

    let messagesElement: any;
    let inputElement: any;
    $: openContextMenu = null;
    $: editingMessage = null;

    onMount(() => {
        messagesElement = document.getElementById('messages');
        inputElement = document.getElementById('send-input')! as HTMLTextAreaElement;

        document.body.addEventListener('click', () => {
            if (openContextMenu) {
                toggleContextMenu(openContextMenu, false);
            }
        });

        document.body.addEventListener('keydown', (event) => {
            if (event.key === 'Escape' && openContextMenu) {
                toggleContextMenu(openContextMenu, false);
            }
        });

        function recalculateHeight() {
            let linebreaks = (inputElement.value.match(/\n/g) || []).length;
            inputElement.style.height = Math.min(24+linebreaks*12, 40) + 'px';
        }

        inputElement.addEventListener('input', () => {
            recalculateHeight()
        });

        keyboardEventStore.subscribe(contact.id.toString(), (event: KeyboardEvent) => {
            if (!event.key || event.ctrlKey || event.altKey || event.metaKey || event.shiftKey) return;
            if (event.key === 'Enter') {
                event.preventDefault();
                submit();
                return;
            }
            if (document.activeElement !== inputElement && !editingMessage) {
                inputElement.focus();
            }
        });

        server.store.subscribe(MESSAGE_DELETED_ID, (deletion) => {
            messages = messages.filter((m) => m.id !== deletion.messageId);
        });

        server.store.subscribe(MESSAGE_EDITED_ID, (edit) => {
            if (edit.editorId !== contact.id) return;
            messages = messages.map((message) => {
                if (message.id === edit.messageId) {
                    return {
                        ...message,
                        content: edit.newContent,
                        edited: true
                    };
                }
                return message;
            });
        })

        server.store.subscribe(MESSAGES_READ_ID, (reading) => {
            if (reading.readerId === contact.id) {
                messages.forEach((message) => {
                    if (reading.messageIds.includes(message.id)) {
                        message.receipt = 2;
                    }
                });
                messages = messages;
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
            clearTimeout(typingTimeout);
            isSelfTyping = false;
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

    function formatReceipt(receipt: number): string {
        switch (receipt) {
            case 0:
                return 'Sent';
            case 1:
                return 'Delivered';
            case 2:
                return 'Read';
        }
    }

    let typingContent = typingMessage;
    function handleTextInput() {
        if (!isSelfTyping) {
            continuousTyping();
        }
    }

    let typingTimeout = undefined;
    function continuousTyping() {
        isSelfTyping = true;
        server.sendPacket(TYPING_REQUEST_ID, {
            contextId: contact.id
        });
        typingTimeout = setTimeout(() => {
            if (typingContent === typingMessage) {
                isSelfTyping = false;
            } else {
                typingContent = typingMessage;
                continuousTyping()
            }
        }, TYPING_DELAY * 0.7);
    }

    function handleMessageContextMenu(event: MouseEvent) {
        event.preventDefault();
        const messageId = (event.currentTarget as HTMLElement).dataset.messageId;
        if (!messageId) return;
        toggleContextMenu(messageId);
    }

    function toggleContextMenu(messageId: string, on: boolean | null = null) {
        if (on === null) {
            openContextMenu = openContextMenu === messageId ? null : messageId;
            setTimeout(() => {
                const contextMenuContent = document.getElementById(`message-context-menu-content-${messageId}`);
                if (contextMenuContent) {
                    const messageText = document.getElementById(`message-text-${messageId}`);
                    if (!messageText) return;
                    const sent = contextMenuContent.classList.contains('sent');
                    contextMenuContent.style.top = `${messageText.getBoundingClientRect().top - 40}px`;
                    if (sent) {
                        contextMenuContent.style.right = `${document.body.clientWidth - messageText.getBoundingClientRect().right}px`;
                    } else {
                        contextMenuContent.style.left = `${messageText.getBoundingClientRect().left}px`;
                    }
                }
            }, 50);
        } else {
            openContextMenu = on ? messageId : null;
        }
    }

    function startEditing(message: any) {
        editingMessage = {
            id: message.id,
            content: message.content
        };
        setTimeout(() => {
            const element = document.getElementById(`edit-input-${message.id}`) as HTMLTextAreaElement;
            element.focus();
        }, 50);
    }

    function submitEdit() {
        if (!editingMessage) return;
        fetch(`${API}/api/messages/${editingMessage.id}`, {
            method: 'PUT',
            headers: {
                'Authorization': 'Bearer ' + token,
            },
            body: JSON.stringify({
                content: editingMessage.content
            })
        }).then((response) => {
            if (response.status === 200) {
                messages = messages.map((message) => {
                    if (message.id === editingMessage.id) {
                        return {
                            ...message,
                            content: editingMessage.content,
                            edited: true
                        };
                    }
                    return message;
                });
                editingMessage = null;
            }
        }).catch((error) => {
            console.error(error);
        });
    }

    function onEditKeydown(event: KeyboardEvent) {
        if (event.key === 'Enter' && !event.shiftKey) {
            event.preventDefault();
            submitEdit();
        }
    }
    
    function deleteMessage(messageId) {
        fetch(`${API}/api/messages/${messageId}`, {
            method: 'DELETE',
            headers: {
                'Authorization': 'Bearer ' + token
            }
        }).then((response) => {
            if (response.status === 204) {
                if (messageId === editingMessage?.id)
                    editingMessage = null;
                messages = messages.filter((message) => message.id !== messageId);
            }
        }).catch((error) => {
            console.error(error);
        });
    }

    const [send, receive] = crossfade({
        duration: (d) => Math.sqrt(d * 200),

        fallback(node, params) {
            const style = getComputedStyle(node);
            const transform = style.transform === 'none' ? '' : style.transform;

            return {
                duration: 600,
                easing: quintOut,
                css: (t) => `
				transform: ${transform} scale(${t});
				opacity: ${t}`
            };
        }
    });

    let alertStore = writable();
</script>

<div class="chat" id="chat-{contact.id}">
    <div class="header">
        <span class="contact-name">{contact?.name}</span>
    </div>
    <div class="messages" id="messages">
        <div class="messages-container" data-relevant="true">
            {#each messages as message,i (message.id)}
                {@const sender = getUserObject(message.user_id)}
                {@const sent = sender.id === user.id}
                <div class="message-container {sent ? 'sent' : 'received'}" in:receive={{key: message.id}} out:send={{key: message.id}}>
                    <div class="message-sender">
                        <span class="message-sender-name">{sender.name}</span>
                    </div>
                    {#if openContextMenu === message.id.toString()}
                        <div class={`message-context-menu ${sent ? 'sent' : 'received'}`} id={`message-context-menu-${message.id}`}>
                            <div class="message-context-menu-blur"></div>
                            <div class="message-context-menu-content {sent ? 'sent' : 'received'}" id={`message-context-menu-content-${message.id}`}>
                                <button class="message-context-menu-item" on:click={() => {
                                    navigator.clipboard.writeText(message.content);
                                    alertStore.set({
                                        type: 'success',
                                        message: 'Copied message to clipboard'
                                    })
                                }}>
                                    <span class="context-menu-tooltip">Copy</span>
                                    <i class="fa-regular fa-copy"></i>
                                </button>
                                {#if sent}
                                <button class="message-context-menu-item" on:click={() => startEditing(message)}>
                                    <span class="context-menu-tooltip">Edit</span>
                                    <i class="fa-regular fa-pen-to-square"></i>
                                </button>
                                <button class="message-context-menu-item delete-item" on:click={() => deleteMessage(message.id)}>
                                    <span class="context-menu-tooltip delete">Delete</span>
                                    <i class="fa-regular fa-trash-can"></i>
                                </button>
                                {/if}
                            </div>
                        </div>
                    {/if}
                    <div class="message-text-container" on:contextmenu={handleMessageContextMenu} data-message-id={message.id} id={`message-text-${message.id}`}>
                        {#if editingMessage?.id === message.id}
                            <form on:submit|preventDefault={submit}>
                                <textarea
                                        id={`edit-input-${message.id}`}
                                        class="message editing"
                                        bind:value={editingMessage.content}
                                        on:keydown={onEditKeydown}
                                />
                            </form>
                        {:else}
                            <span
                                    class={`message ${sent ? 'sent' : 'received'}`}
                                    data-only-emojis={isMessageMadeOfOnlyEmojis(message.content)}
                                    data-emoji-count={countEmojis(message.content)}
                            >{@html message.content.replace(/\n/g, '<br>')}</span>
                        {/if}
                    </div>
                    <div class="message-details {sent ? 'sent' : 'received'}">
                        {#if message.edited || !messages[i+1] || messages[i+1].user_id !== message.user_id}
                            {#if message.edited}
                                <span class="edited-text">
                                    <i class="fa-solid fa-pen"></i>
                                    Edited
                                </span>
                                •
                            {/if}
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
    </div>
    {#if typing}
        <div class="typing-container">
            <img src="/assets/typing.svg" alt="Typing..." class="typing-gif"/>
            <span class="typing">{contact.name} is currently typing...</span>
        </div>
    {/if}
    <div class="alert-container">
        <div class="alert-position">
            <Alert alertStore={alertStore}/>
        </div>
    </div>
    <div class="send-container">
        <div class="send-input-container">
            <textarea
                    id="send-input"
                    class="send-input"
                    rows="1"
                    on:input={handleTextInput}
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
        max-width: 200px;
        font-family: 'DM Sans', sans-serif;
        font-size: 17px;
        text-wrap: auto;
        word-wrap: normal;
        overflow-wrap: normal;
        word-break: break-word;
    }
    .message.editing {
        resize: none;
        padding: 16px;
        max-width: 600px;
        outline: none;
        border: 8px solid var(--chat-sender-color);
        background-color: var(--chat-sender-color-contrast);
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
        display: flex;
        align-items: center;
        justify-content: left;
        gap: 4px;
        width: 600px;
        font-family: 'DM Sans', sans-serif;
        font-size: 12px;
        color: var(--message-details);
    }
    .message-details.sent {
        justify-content: right !important;
    }
    .edited-text {
        display: flex;
        align-items: center;
        font-size: 12px;
        font-weight: 800;
        gap: 8px;
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
    .typing-container {
        display: flex;
        align-items: center;
        gap: 16px;
        justify-content: center;
        width: 100%;
        height: 40px;
        background-color: var(--light-contrast);
    }
    .typing {
        font-family: 'DM Sans', sans-serif;
        font-size: 16px;
        color: var(--con);
        font-style: italic;
        animation: typing 1s infinite;
    }

    @keyframes typing {
        0% {
            opacity: 0.3;
        }
        50% {
            opacity: 0.8;
        }
        100% {
            opacity: 0.3;
        }
    }

    .typing-gif {
        height: 100%;
        aspect-ratio: 1/1;
    }

    .message-context-menu {
        position: absolute;
        display: flex;
        top: 0;
        right: 0;
        width: 100%;
        height: 100%;
        border: none;
        border-radius: 512px;
    }

    .message-context-menu.sent {
        align-items: flex-end;
        justify-content: right;
    }

    .message-context-menu-content {
        display: flex;
        position: absolute;
        z-index: 6;
        border-radius: 512px;
        background-color: var(--heavy-constrast);
        animation: content-open 0.4s forwards;
    }

    .message-context-menu-content.sent {
        align-items: flex-end;
        justify-content: flex-end;
    }

    @keyframes content-open {
        0% {
            transform: scaleX(0);
        }
        100% {
            transform: scaleX(1);
        }
    }

    .message-context-menu-item {
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 4px;
        font-family: 'DM Sans', sans-serif;
        font-size: 24px;
        color: var(--text-color);
        background-color: transparent;
        border: none;
        width: 64px;
        border-radius: 50%;
        margin-bottom: auto;
        aspect-ratio: 1/1;
        cursor: pointer;
        transition: all 0.4s;
    }
    .message-context-menu-item:hover {
        background-color: var(--background);
    }

    .message-context-menu-item:hover .context-menu-tooltip {
        animation: tooltip 0.1s forwards;
        display: block;
    }

    .context-menu-tooltip {
        display: none;
        position: absolute;
        background-color: var(--background);
        border-radius: 512px;
        padding: 4px 12px;
        font-size: 14px;
    }

    @keyframes tooltip {
        0% {
            bottom: 64px;
            opacity: 0;
        }
        100% {
            bottom: 80px;
            opacity: 1;
        }
    }

    .context-menu-tooltip.delete {
        color: #d95252;
    }

    .delete-item {
        color: #d95252;
    }
    .delete-item:hover {
        color: var(--text-color);
        background-color: #d95252;
    }
    .message-context-menu-blur {
        position: absolute;
        top: 0;
        right: 0;
        width: 100%;
        height: 100%;
        z-index: 6;
        animation: blur 0.4s forwards;
    }

    @keyframes blur {
        0% {
            backdrop-filter: blur(0);
        }
        100% {
            backdrop-filter: blur(10px);
        }
    }

    .alert-container {
        position: relative;
        width: 100%;
        height: 0;
    }
    .alert-position {
        position: absolute;
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    }
</style>