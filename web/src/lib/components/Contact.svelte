<script lang="ts">
    import {onMount} from "svelte";
    import {CONTEXT_READ_ID} from "$lib/network/message.ts";
    import server from "$lib/network/server.ts";

    export let selfId: number;
    export let picture: string;
    export let user: any;
    export let selected: any | null;
    export let messageStore: any;
    export let typing: boolean;

    $: lastMessage = user.last_message;

    $: isSelected = selected && selected.id === user.id;
    $: unreadCount = user.unread_count || 0;

    onMount(() => {
        messageStore.subscribe((message) => {
            if (!message) return;
            else if (message.userId === selfId && message.context === user.id) {
                lastMessage = message;
            } else if (message.userId === user.id && message.context === selfId) {
                lastMessage = message;
                if (!isSelected) {
                    unreadCount++;
                }
            }
        });
    });

    function handleMouseMove(event: MouseEvent) {
        const contact = event.currentTarget as HTMLButtonElement;
        const rect = contact.getBoundingClientRect();
        const x = event.clientX - rect.left;
        const y = event.clientY - rect.top;

        contact.style.setProperty('--x', x + 'px');
        contact.style.setProperty('--y', y + 'px');
    }

    function formatLastMessage(lastMessage): string {
        const lastMessageContent = lastMessage?.content;
        if (!lastMessageContent) {
            return 'No messages yet.';
        }

        if (lastMessageContent.length > 30) {
            return lastMessageContent.slice(0, 30) + '...';
        }
        return lastMessageContent;
    }

    function handleClick() {
        if (selected && selected.id === user.id) {
            return;
        }
        server.sendPacket(
            CONTEXT_READ_ID,
            { contextId: user.id }
        )
        unreadCount = 0;
        selected = user;
        history.pushState(null, '', `/app/contacts/${user.id}`);
    }
</script>

<button class="contact" on:click={() => handleClick()} on:mousemove={handleMouseMove} data-selected={isSelected}>
    <div class="contact-content" on:mousemove={handleMouseMove} data-selected={isSelected}>
        <div class="top">
            <img src={picture} alt={`${user.name}'s profile picture`} class="photo"/>
            <div class="text">
                <div class="upper-text">
                    <div class="identifier">
                        <span class="name">{user.name}</span>
                        <span class="username">@{user.username}</span>
                    </div>
                    {#if unreadCount > 0}
                        <span class="unread-count">{unreadCount}</span>
                    {/if}
                </div>
                <span class="biography">A free thinker roaming Earth.</span>
            </div>
        </div>
        <div class="last-message-container">
            {#if typing}
                <img src="/assets/typing.svg" alt="Typing..." class="typing-gif"/>
                <span class="last-message typing">Typing...</span>
            {:else}
                <span class="last-message">{formatLastMessage(lastMessage)}</span>
            {/if}
        </div>
    </div>
</button>

<style>
    .contact {
        font-family: 'DM Sans', sans-serif;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 90%;
        min-height: 75px;
        border: none;
        outline: none;
        text-decoration: none;
        border-radius: 6px;
        background-color: var(--light-contrast);
        transition: all 0.2s;
        user-select: none;
        cursor: pointer;
        position: relative;
    }

    .contact[data-selected="true"] {
        background-color: var(--selected-contact);
        cursor: default;
        box-shadow: 0 0 2px black;
        transform: translateY(-2px);
    }

    .contact[data-selected="true"] .contact-content {
        background-color: inherit;
    }

    .contact[data-selected="true"] .contact-content::before {
        opacity: 0;
    }

    .contact[data-selected="true"]::before {
        opacity: 0;
        background: var(--selected-contact) none;
    }

    .contact-content {
        position: relative;
        z-index: 2;
        display: flex;
        width: 100%;
        height: 100%;
        padding: 10px 4px;
        border-radius: 4px;
        flex-direction: column;
        background-color: var(--light-contrast);
    }

    .contact-content::before {
        position: absolute;
        left: 0;
        top: 0;
        border-radius: inherit;
        content: "";
        z-index: 1;
        opacity: 0;
        width: calc(100%);
        height: calc(100%);
        background: radial-gradient(
                1200px circle at var(--x) var(--y),
                rgba(var(--delicate-hover-contrast), 0.07),
                transparent 20%
        );
        transition: opacity 0.6s;
    }

    .contact-content:hover::before {
        opacity: 1;
    }

    .top {
        display: flex;
        align-items: center;
    }

    .contact::before {
        position: absolute;
        border-radius: inherit;
        align-self: center;
        justify-self: center;
        content: "";
        z-index: 1;
        opacity: 0;
        margin-bottom: 1px;
        width: calc(100% + 2px);
        height: calc(100%);
        background: radial-gradient(
            800px circle at var(--x) var(--y),
            rgba(var(--delicate-hover-contrast), 0.3),
            transparent 40%
        );
        transition: opacity 0.6s;
    }

    .contact:hover::before {
        opacity: 1;
    }

    .photo {
        width: 50px;
        height: 50px;
        border-radius: 50%;
        margin: 10px;
        object-fit: cover;
    }

    .text {
        width: 100%;
        display: flex;
        align-items: flex-start;
        flex-direction: column;
    }

    .upper-text {
        position: relative;
        display: flex;
        width: 100%;
    }

    .unread-count {
        position: absolute;
        color: white;
        font-weight: 600;
        font-size: 18px;
        font-family: 'DM Sans', sans-serif;
        border-radius: 100%;
        aspect-ratio: 1/1;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 24px;
        background-color: #e26666;
        right: 12px;
    }

    .name {
        font-size: 16px;
        font-weight: 600;
        color: var(--text-color);
    }

    .identifier {
        display: flex;
        align-items: center;
        gap: 4px;
    }

    .username {
        font-weight: 400;
        color: var(--blurred-username);
        font-size: 13px;
    }

    .biography {
        font-weight: 400;
        color: var(--biography-contact);
        font-size: 13px;
    }

    .last-message-container {
        display: flex;
        align-items: center;
        justify-content: flex-start;
        gap: 12px;
        margin: 6px 10px;
    }

    .last-message {
        font-weight: 500;
        text-align: left;
        color: var(--text-color);
    }

    .typing {
        font-style: italic;
        color: red;
        animation: typing 1s infinite;
    }

    .typing-gif {
        aspect-ratio: 1/1;
        height: 24px;
    }

    @keyframes typing {
        0% {
            color: #ccc;
        }
        50% {
            color: #878787;
        }
        100% {
            color: #ccc;
        }
    }
</style>