<script>
    import EmojiMenu from "$lib/components/EmojiMenu.svelte";
    import {onDestroy, onMount} from "svelte";
    import Message from "$lib/components/Message.svelte";
    import {
        addReaction,
        deleteReaction,
        editMessage,
        excludeMessage, fetchContact,
        fetchMessages,
        sendMessage
    } from "$lib/network/api.ts";
    import server from "$lib/network/server.ts";
    import {
        CHANNEL_READ_ID,
        MESSAGE_CREATED_ID,
        MESSAGE_DELETED_ID,
        MESSAGE_EDITED_ID, MESSAGES_READ_ID,
        REACTION_ADDED_ID, REACTION_REMOVED_ID, TYPING_REQUEST_ID
    } from "$lib/network/message.ts";
    import Alert from "$lib/components/Alert.svelte";
    import {writable} from "svelte/store";
    import {TYPING_DELAY} from "$lib/constants.ts";
    import {createCrossfade} from "$lib/animation/crossfade.ts";

    export let token;
    export let user;
    export let channelId;
    export let typingUsers;

    let typingMessage = '';

    let messagesElement;
    let inputElement;

    let messages = [];
    $: openContextMenu = null;
    $: editingMessage = null;
    $: replyingTo = null;
    $: reactingTo = null;
    $: addingEmoji = false;

    let userTyping = false;
    let typingTimeout = null;
    let alertStore = writable();

    const [send, receive] = createCrossfade(200, 600);

    onMount(() => {
        messagesElement = document.getElementById('messages');
        inputElement = document.getElementById('send-input');
        loadMessages();
        document.body.addEventListener('keydown', handleKeydown);

        server.store.subscribe(MESSAGE_CREATED_ID, onMessageCreate);
        server.store.subscribe(MESSAGE_EDITED_ID, onMessageEdit);
        server.store.subscribe(MESSAGE_DELETED_ID, onMessageDelete);
        server.store.subscribe(MESSAGES_READ_ID, onBulkMessagesRead);
        server.store.subscribe(REACTION_ADDED_ID, onReactionAdd);
        server.store.subscribe(REACTION_REMOVED_ID, onReactionRemoval);
    });

    onDestroy(() => {
        document.body.removeEventListener('keydown', handleKeydown);

        server.store.unsubscribe(MESSAGE_CREATED_ID, onMessageCreate);
        server.store.unsubscribe(MESSAGE_EDITED_ID, onMessageEdit);
        server.store.unsubscribe(MESSAGE_DELETED_ID, onMessageDelete);
        server.store.unsubscribe(MESSAGES_READ_ID, onBulkMessagesRead);
        server.store.unsubscribe(REACTION_ADDED_ID, onReactionAdd);
        server.store.unsubscribe(REACTION_REMOVED_ID, onReactionRemoval);
    });

    function loadMessages() {
        fetchMessages(token, channelId).then((fetchedMessages) => {
            messages = fetchedMessages.reverse();
        }).then(() => {
            scrollToBottom();
        }).catch((error) => {
            console.error(error);
        });
    }

    function handleKeydown(event) {
        if (event.key === 'Escape') {
            clearState()
        }
        // Ignore keydown events if the user is already focused on an input element

        if (!event.key || event.ctrlKey || event.altKey || event.metaKey || event.shiftKey) return;
        if (event.key === 'Enter' && document.activeElement === inputElement) {
            event.preventDefault();
            submit();
            return;
        }
        if (document.activeElement.tagName === 'INPUT' || document.activeElement.tagName === 'TEXTAREA')
            return;
        if (!editingMessage && !openContextMenu && !addingEmoji) {
            inputElement.focus();
        }
    }

    function onMessageCreate(creation) {
        if (!creation) return;
        let message = creation.message;
        if (message.channel_id === channelId && message.user_id !== user.id) {
            messages = [...messages, message];
            setTimeout(() => {
                scrollToBottom();
            }, 50);
        }
    }

    function onMessageEdit(edit) {
        if (!edit) return;
        if (edit.channel_id !== channelId || edit.editor_id === user.id) return;
        messages = messages.map((message) => {
            if (message.id === edit.message_id) {
                return {
                    ...message,
                    edited: true,
                    content: edit.new_content
                };
            }
            return message;
        });
    }

    function onMessageDelete(deletion) {
        if (!deletion) return;
        // TODO: filter user out
        if (deletion.channel_id !== channelId) return;
        messages = messages.filter((m) => m.id !== deletion.message_id);
    }

    function onReactionAdd(reaction) {
        console.log(reaction);
        if (!reaction || reaction.channel_id !== channelId) return;
        const message = messages.find((m) => m.id === reaction.message_id);
        if (!message || reaction.user_id === user.id) return; // TODO: fetch message
        locallyAddReaction(message, reaction.user_id, reaction.reaction_id, reaction.reaction_count, reaction.emoji);
    }

    function onReactionRemoval(reaction) {
        console.log(reaction);
        if (!reaction || reaction.channel_id !== channelId) return;
        const message = messages.find((m) => m.id === reaction.message_id);
        if (!message) return; // TODO: fetch message
        locallyRemoveReaction(message, reaction.user_id, reaction.reaction_id, reaction.reaction_count);
    }

    function onBulkMessagesRead(reading) {
        console.log("New reading", reading);
        // TODO: fix receipts system
        // if (reading.readerId === contact.id) {
        //     messages.forEach((message) => {
        //         if (reading.messageIds.includes(message.id)) {
        //             message.receipt = 2;
        //         }
        //     });
        //     messages = messages;
        // }
    }

    function scrollToBottom() {
        messagesElement.scrollTo(0, messagesElement.scrollHeight);
    }

    function submit() {
        let trimmed = typingMessage.trim();
        if (!typingMessage || trimmed.length === 0) return;
        messagesElement.scrollTo(0, messagesElement.scrollHeight);
        sendMessage(token, channelId, trimmed, replyingTo?.id).then((message) => {
            clearTimeout(typingTimeout);
            clearState();
            userTyping = false;
            messages = [...messages, message];
            typingMessage = '';
            inputElement.style.height='24px';
        }).then(() => {
            scrollToBottom();
        }).catch((error) => {
            console.error(error);
        });
    }

    function submitEdit() {
        if (!editingMessage) return;
        editMessage(token, channelId, editingMessage.id, editingMessage.content).then((data) => {
            if (data) {
                messages = messages.map((message) => {
                    if (message.id === editingMessage.id) {
                        return data;
                    }
                    return message;
                });
                editingMessage = null;
            }
        }).catch((error) => {
            console.error(error);
        });
    }

    function clearState() {
        console.log("Clearing state");
        openContextMenu = null;
        editingMessage = null;
        replyingTo = null;
        reactingTo = null;
        addingEmoji = false;
    }

    function reactEmoji(message, emoji) {
        addReaction(token, channelId, message.id, emoji).then((data) => {
            if (data) {
                locallyAddReaction(message, user.id, data.reaction_id, data.reaction_count, emoji);
            }
        }).catch((error) => {
            console.error(error);
        });
    }

    function removeReaction(message, reactionId) {
        let reaction = message.reactions.find((reaction) => reaction.reaction_id === reactionId);
        if (!reaction) return;
        deleteReaction(token, channelId, message.id, reactionId).then((response) => {
            if (response.status === 204) {
                locallyRemoveReaction(message, user.id, reactionId, reaction.count - 1);
            }
        }).catch((error) => {
            console.error(error);
        });
    }

    function locallyAddReaction(message, userId, reactionId, reactionCount, emoji) {
        let existingReaction = message.reactions ? message.reactions.find((reaction) => reaction.reaction_id === reactionId) : null;

        if (existingReaction) {
            const newMessage = message = {
                ...message,
                reactions: message.reactions.map((reaction) => {
                    if (reaction.emoji === emoji) {
                        return {
                            ...reaction,
                            reaction_id: reactionId,
                            count: reactionCount,
                            me: reaction.me || userId === user.id
                        };
                    }
                    return reaction;
                })
            };
            messages = messages.map((m) => m.id === message.id ? newMessage : m);
        } else {
            const newMessage = message = {
                ...message,
                reactions: [
                    ...(message.reactions || []),
                    {
                        reaction_id: reactionId,
                        emoji: emoji,
                        count: reactionCount,
                        me: userId === user.id
                    }
                ]
            };
            messages = messages.map((m) => m.id === message.id ? newMessage : m);
        }
    }

    function locallyRemoveReaction(message, userId, reactionId, reactionCount) {
        if (reactionCount > 0) {
            const newMessage = message = {
                ...message,
                reactions: message.reactions.map((reaction) => {
                    if (reaction.reaction_id === reactionId) {
                        return {
                            ...reaction,
                            count: reactionCount,
                            me: reaction.me && userId !== user.id
                        };
                    }
                    return reaction;
                })
            };
            messages = messages.map((m) => m.id === message.id ? newMessage : m);
        } else {
            const newMessage = message = {
                ...message,
                reactions: message.reactions.filter((reaction) => reaction.reaction_id !== reactionId)
            };
            messages = messages.map((m) => m.id === message.id ? newMessage : m);
        }
    }

    function calculateAddingEmojiPickerPosition() {
        const emojiAddButton = document.getElementById('add-emoji-button');
        if (!emojiAddButton) return {x:100, y:100};
        const rect = emojiAddButton.getBoundingClientRect();
        console.log(rect.bottom);
        const y =  window.innerHeight - rect.bottom + 80;
        return {x: rect.left, y};
    }

    let typingContent = typingMessage;
    function handleTextInput() {
        if (!userTyping) {
            continuousTyping();
        }
    }

    function continuousTyping() {
        userTyping = true;
        server.sendPacket(TYPING_REQUEST_ID, {
            channel_id: channelId
        });
        typingTimeout = setTimeout(() => {
            if (typingContent === typingMessage) {
                userTyping = false;
            } else {
                typingContent = typingMessage;
                continuousTyping()
            }
        }, TYPING_DELAY * 0.7);
    }

    function getUserObject(id, then) {
        if (id === user.id) {
            then(user);
        } else {
            return fetchContact(token, id).then((contact) => {
                then(contact);
            });
        }
    }

    function deleteMessage(messageId) {
        excludeMessage(token, channelId, messageId).then((response) => {
            if (response.status === 204) {
                messages = messages.filter((message) => message.id !== messageId);
            }
        }).catch((error) => {
            console.error(error);
        });
    }
</script>

<div class="chat">
    {#if reactingTo}
        <EmojiMenu onClick={(emoji) => {
            reactEmoji(reactingTo, emoji.emoji);
            clearState();
        }} center={true}/>
    {:else if addingEmoji}
        {@const position = calculateAddingEmojiPickerPosition()}
        <EmojiMenu onClick={(emoji) => {
            typingMessage += emoji.emoji;
        }} bottom={position.y} left={position.x}/>
    {/if}
    <div class="messages" id="messages">
        <div class="messages-container" data-relevant="true">
            {#each messages as message,i (message.id)}
                <div in:receive={{key: message.id}} out:send={{key: message.id}}>
                    <Message
                            user={user}
                            message={message}
                            followingMessage={messages[i + 1]}
                            reactEmoji={reactEmoji}
                            removeReaction={removeReaction}
                            clearState={clearState}
                            messageRepliesTo={messages.find((m) => m.id === message.reply_to || m.id === message.reply_to?.id)}
                            submitEdit={submitEdit}
                            excludeMessage={deleteMessage}
                            bind:editingMessage={editingMessage}
                            bind:reactingTo={reactingTo}
                            bind:replyingTo={replyingTo}
                            bind:openContextMenu={openContextMenu}
                            bind:alertStore={alertStore}
                    />
                </div>
            {/each}
        </div>
    </div>
    <div class="footer">
        <div class="typing-container">
            {#if typingUsers.length > 0}
                <img src="/assets/typing.svg" alt="Typing..." class="typing-gif"/>
                {#if typingUsers.length === 1}
                    <span class="typing">{typingUsers[0].name} is currently typing...</span>
                {:else}
                    <span class="typing">Several people are currently typing...</span>
                {/if}
            {/if}
        </div>
        <div class="alert-container">
            <div class="alert-position">
                <Alert alertStore={alertStore}/>
            </div>
        </div>
        <div class="send-container">
            {#if replyingTo}
                <div class="replying-to">
                    <span class="replying-to-text">Replying to:</span>
                    <span class="replying-to-content">{replyingTo.content}</span>
                    <button class="replying-to-close" on:click={() => replyingTo = null}>
                        <i class="fa-solid fa-xmark"></i>
                    </button>
                </div>
            {/if}
            <button class="input-add-emoji" on:click={() => addingEmoji = !addingEmoji} id="add-emoji-button">
                <i class="fa-regular fa-face-grin-stars"></i>
            </button>
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
</div>

<style>
    .chat {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
        height: 100%;
        border-left: 1px solid var(--thin-border);
        background-color: var(--light-contrast);
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
        height: 75vh;
        padding-top: 32px;
        overflow-x: hidden;
        overflow-y: auto;
    }
    .messages-container {
        display: flex;
        flex-direction: column;
        width: 95%;
        margin-bottom: 40px;
    }
    .footer {
        width: 92.5%;
        height: 15vh;
    }
    .send-container {
        position: relative;
        display: flex;
        align-items: center;
        width: 100%;
        height: 60px;
        border-radius: 8px;
        box-shadow: 0 0 2px 0 var(--reply-color);
        padding: 4px;
        background-color: var(--background);
        margin-bottom: 48px;
    }
    .replying-to {
        position: absolute;
        bottom: 64px;
        border-radius: 16px 16px 0 0;
        left: -1px;
        font-family: 'DM Sans', sans-serif;
        font-size: 12px;
        height: 28px;
        width: calc(100% - 19px);
        display: flex;
        align-items: center;
        padding-left: 20px;
        z-index: 2;
        background-color: var(--heavy-constrast);
        animation: reply 0.4s forwards;
    }
    .replying-to-text {
        color: var(--text-color);
        font-weight: 600;
        font-family: 'Roboto', sans-serif;
    }
    .replying-to-content {
        color: var(--reply-to);
        font-style: italic;
        font-family: 'Roboto', sans-serif;
        margin-left: 4px;
    }
    .replying-to-close {
        border: none;
        background-color: transparent;
        color: #e56c6c;
        margin-right: 12px;
        cursor: pointer;
        font-size: 20px;
        margin-left: auto;
    }
    @keyframes reply {
        0% {
            transform: translateY(100%);
        }
        100% {
            transform: translateY(0);
        }
    }
    .send-input-container {
        position: relative;
        z-index: 3;
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

    .alert-container {
        position: relative;
        width: 100%;
        height: 0;
    }
    .alert-position {
        position: absolute;
        z-index: 8;
        width: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    .input-add-emoji {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 80%;
        aspect-ratio: 1/1;
        border-radius: 50%;
        background-color: var(--light-contrast);
        color: var(--text-color);
        margin-left: 6px;
        border: none;
        font-size: 24px;
        cursor: pointer;
        transition: background-color 0.3s;
    }
    .input-add-emoji:hover {
        background-color: var(--input-add-emoji-hover);
    }
</style>