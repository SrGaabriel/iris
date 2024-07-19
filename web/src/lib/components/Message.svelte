<script>
    import {getTimestamp, getTimestampFormatted} from "$lib/util/snowflake.ts";
    import {countEmojis, isMessageMadeOfOnlyEmojis} from "$lib/util/emojis.ts";

    export let user;
    export let contact;
    export let sender;
    export let sent;
    export let openContextMenu;
    export let message;
    export let messageRepliesTo;
    export let followingMessage;
    export let clearState;
    export let replyingTo;
    export let reactingTo;
    export let editingMessage;
    export let reactEmoji;
    export let removeReaction;
    export let submitEdit;

    function handleMessageContextMenu(event) {
        event.preventDefault();
        toggleContextMenu(true);
    }

    function toggleContextMenu(on) {
        if (on) {
            openContextMenu = message.id;
            setContextMenuPosition();
        } else {
            openContextMenu = null;
        }
    }

    function setContextMenuPosition() {
        setTimeout(() => {
            const contextMenuContent = document.getElementById(`message-context-menu-content-${message.id}`);
            if (contextMenuContent) {
                const messageText = document.getElementById(`message-text-${message.id}`);
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
    }

    function formatReceipt(receipt) {
        switch (receipt) {
            case 0: return 'Sent';
            case 1: return 'Delivered';
            case 2: return 'Read';
        }
    }

    function startEditing() {
        editingMessage = message;
        const input = document.getElementById(`edit-input-${message.id}`);
        if (input) {
            input.focus();
        }
    }

    function onEditKeydown(event) {
        if (event.key === 'Enter' && !event.shiftKey) {
            event.preventDefault();
            submitEdit();
        }
    }

    function deleteMessage() {
        if (confirm('Are you sure you want to delete this message?')) {
            // deleteMessage(message);
        }
    }
</script>

<div class="message-container {sent ? 'sent' : 'received'}">
    <div class="message-sender">
        <span class="message-sender-name">{sender.name}</span>
    </div>
    {#if openContextMenu === message.id.toString()}
        <div class={`message-context-menu ${sent ? 'sent' : 'received'}`} id={`message-context-menu-${message.id}`}>
            <div class="message-context-menu-blur" on:click={() => {
                clearState();
            }}></div>
            <div class="message-context-menu-content {sent ? 'sent' : 'received'}" id={`message-context-menu-content-${message.id}`}>
                <button class="message-context-menu-item" on:click={() => { reactingTo = message; }}>
                    <span class="context-menu-tooltip">React</span>
                    <i class="fa-regular fa-face-grin-tongue-wink"></i>
                </button>
                <button class="message-context-menu-item" on:click={() => {
                    clearState();
                    replyingTo = message;
                }}>
                    <span class="context-menu-tooltip">Reply</span>
                    <i class="fa-solid fa-reply"></i>
                </button>
                <button class="message-context-menu-item" on:click={() => {
                    clearState();
                    navigator.clipboard.writeText(message.content);
                    // alertStore.set({
                    //     type: 'success',
                    //     message: 'Copied message to clipboard'
                    // });
                }}>
                    <span class="context-menu-tooltip">Copy</span>
                    <i class="fa-regular fa-copy"></i>
                </button>
                {#if sent}
                    <button class="message-context-menu-item" on:click={() =>
                    {clearState(); startEditing()}}
                    >
                        <span class="context-menu-tooltip">Edit</span>
                        <i class="fa-regular fa-pen-to-square"></i>
                    </button>
                    <button class="message-context-menu-item delete-item" on:click={() =>
                    {clearState(); deleteMessage()}}
                    >
                        <span class="context-menu-tooltip delete">Delete</span>
                        <i class="fa-regular fa-trash-can"></i>
                    </button>
                {/if}
            </div>
        </div>
    {/if}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class={`message-text-container ${sent ? 'sent' : 'received'}`} on:contextmenu={handleMessageContextMenu} data-message-id={message.id} id={`message-text-${message.id}`}>
          {#if editingMessage?.id === message.id}
              <textarea
                  id={`edit-input-${message.id}`}
                  class="message editing"
                  bind:value={editingMessage.content}
                  on:keydown={onEditKeydown}
              />
          {:else}
              {#if message.reply_to}
                  <div class={`message-reply ${sent ? 'sent' : 'received'}`} on:click={() => {
                      if (messageRepliesTo) {
                          const element = document.getElementById(`message-text-${messageRepliesTo.id}`);
                          element.scrollIntoView({behavior: 'smooth', block: 'center'});
                      }
                  }}>
                      <div class="message-reply-header">
                          <span class="message-reply-sender-name">{messageRepliesTo ? (messageRepliesTo.user_id === user.id ? user.name : contact.name) : 'Unknown'}</span>
                          {#if messageRepliesTo}
                              •
                              <span class="message-reply-header-text">{getTimestampFormatted(getTimestamp(messageRepliesTo.id))}</span>
                          {/if}
                          {#if messageRepliesTo?.edited}
                              •
                              <span class="message-reply-edited-text">
                                  <i class="fa-solid fa-pen"></i>
                                  Edited
                              </span>
                          {/if}
                      </div>
                      {#if messageRepliesTo}
                          <span class="message-reply-content">{messageRepliesTo.content}</span>
                      {:else}
                          <span class="message-reply-content deleted">
                              <i class="fa-regular fa-trash-can"></i>
                              This message was either lost or deleted.
                          </span>
                      {/if}
                  </div>
              {/if}
              <span
                      class={`message ${sent ? 'sent' : 'received'}`}
                      data-only-emojis={isMessageMadeOfOnlyEmojis(message.content)}
                      data-emoji-count={countEmojis(message.content)}
              >
              {@html message.content.replace(/\n/g, '<br>')}
          </span>
          {/if}
    </div>
    {#if message?.reactions?.length > 0}
        <div class="reactions">
            {#each message.reactions as reaction}
                <button class={`reaction ${reaction.me ? 'reacted' : ''}`} on:click={() => {
                        clearState();
                        if (reaction.me) {
                            removeReaction(message, reaction.reaction_id);
                        } else {
                            reactEmoji(message, reaction.emoji);
                        }
                    }}>{reaction.emoji} {reaction.count}</button>
            {/each}
        </div>
    {/if}
    <div class="message-details {sent ? 'sent' : 'received'}">
        {#if message.edited || !followingMessage || followingMessage.user_id !== message.user_id}
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

<style>
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
        border-radius: 6px;
        padding: 10px 10px 10px 10px;
    }
    .message-text-container.sent {
        background-color: var(--chat-sender-color);
    }
    .message-text-container.received {
        background-color: var(--chat-receiver-color);
    }
    .message-sender-name {
        font-family: 'DM Sans', sans-serif;
        color: var(--message-sender-name);
        font-size: 14px;
        margin-left: 2px;
        font-weight: 500;
    }
    .message-reply {
        display: flex;
        border-radius: 4px;
        flex-direction: column;
        padding: 7px 10px;
        margin-right: auto;
        max-width: 300px;
        transition: background-color 0.4s;
        cursor: pointer;
        margin-bottom: 8px;
    }
    .message-reply.sent {
        border-left: 4px solid var(--chat-sender-color-dark-contrast);
        background-color: var(--chat-sender-color-contrast);
    }
    .message-reply.received {
        border-left: 4px solid var(--chat-receiver-color-dark-contrast);
        background-color: var(--chat-receiver-color-contrast);
    }
    .message-reply:hover {
        user-select: none;
        -webkit-user-select: none;
        -moz-user-select: none;
    }
    .message-reply.sent:hover {
        background-color: var(--chat-sender-color-contrast-hover);
    }
    .message-reply.received:hover {
        background-color: var(--chat-receiver-color-contrast-hover);
    }
    .message-reply-header {
        display: flex;
        gap: 6px;
        align-items: center;
    }
    .message-reply-header-text {
        font-size: 14px;
        font-family: 'DM Sans', sans-serif;
        color: var(--reply-color-details);
    }
    .message-reply-edited-text {
        display: flex;
        gap: 6px;
        align-items: center;
        font-size: 12px;
        font-weight: 600;
        font-family: 'DM Sans', sans-serif;
        color: var(--reply-color-details);
    }
    .message-reply-sender-name {
        font-family: 'DM Sans', sans-serif;
        color: var(--reply-color);
        font-size: 14px;
        margin-left: 2px;
        font-weight: 600;
    }
    .message-reply-content {
        font-family: 'DM Sans', sans-serif;
        font-size: 14px;
        color: var(--reply-color-content);
        text-wrap: wrap;
        word-wrap: normal;
        overflow-wrap: normal;
        word-break: break-word;
        margin-top: 4px;
    }
    .message-reply-content.deleted {
        display: flex;
        align-items: center;
        gap: 8px;
        font-style: italic;
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
        animation: blur 0.8s forwards;
    }

    @keyframes blur {
        0% {
            backdrop-filter: blur(0);
        }
        100% {
            backdrop-filter: blur(6px);
        }
    }

    .reactions {
        display: flex;
        gap: 8px;
        margin-top: 4px;
    }
    .reaction {
        border-radius: 6px;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 48px;
        height: 32px;
        font-weight: 600;
        color: var(--reaction-counter);
        font-family: 'Roboto', sans-serif;
        font-size: 15px;
        background-color: var(--heavy-constrast);
        border: 2px solid transparent;
        cursor: pointer;
        transition: all 0.2s;
    }
    .reaction:hover {
        border: 2px solid var(--reaction-counter);
    }
    .reaction.reacted {
        background-color: var(--reaction-selected-background);
        border: 2px solid var(--reaction-selected-border);
    }
</style>