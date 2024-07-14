<script lang="ts">
    import Contact from "$lib/components/Contact.svelte";
    import {onMount} from "svelte";
    import server, {API} from "../../../interaction/server.ts";
    import Chat from "$lib/components/Chat.svelte";
    import {writable} from "svelte/store";
    import ThemeToggle from "$lib/components/ThemeToggle.svelte";
    import TargetedStore from '../../../util/targetedStore.ts';
    import {CONTEXT_READ_ID, MESSAGES_READ_ID, TEXT_MESSAGE_ID} from "../../../interaction/message.ts";

    export let data;
    let contacts: any[] = [];
    $: selectedContact = null;
    let messageStore = writable();
    let keydownStore = new TargetedStore();

    onMount(() => {
        document.addEventListener('keydown', (event) => {
            if (!selectedContact) return;
            if (event.ctrlKey || !event.altKey || !event.metaKey) return;
            if (event.key === 'Enter' || (isCharacterKeyPress(event) && event.key.length === 1)) {
                keydownStore.dispatch(selectedContact.id.toString(), event.key);
            }
        });

        fetch(`${API}/api/contacts/@me`, {
            method: 'GET',
            headers: {
                'Authorization': 'Bearer ' + data.token
            }
        }).then(response => response.text()).then((text) => JSONbig.parse(text)).then(contact_list => {
            contacts = contact_list;
        });
        server.connect(data.token);
        server.store.subscribeAll((packetId: number, message: any) => {
            switch (packetId) {
                case TEXT_MESSAGE_ID:
                    messageStore.set(message);
                    if (selectedContact && message.userId === selectedContact.id) {
                        server.sendPacket(CONTEXT_READ_ID, {contextId: selectedContact.id});
                    }
                    break;
                case MESSAGES_READ_ID:
                    break;
                default:
                    console.log(`Unknown packet id: ${packetId}`);
            }
        });
    })

    function isCharacterKeyPress(evt) {
        if (typeof evt.which == "undefined") {
            // This is IE, which only fires keypress events for printable keys
            return true;
        } else if (typeof evt.which == "number" && evt.which > 0) {
            // In other browsers except old versions of WebKit, evt.which is
            // only greater than zero if the keypress is a printable key.
            // We need to filter out backspace and ctrl/alt/meta key combinations
            return !evt.ctrlKey && !evt.metaKey && !evt.altKey && evt.which != 8;
        }
        return false;
    }
</script>

<div class="page">
    <div class="header">
        <div class="icon-space">
        </div>
        <div class="theme-toggle-container">
            <ThemeToggle/>
        </div>
        <div class="self-profile">
            <img src="/assets/no_profile_picture.jpg" alt="Your profile picture" class="photo"/>
            <div class="self-identifier">
                <div class="self-identifier-name">
                    <span class="self-name">{data.user.name}</span>
                    <span class="self-username">@{data.user.username}</span>
                </div>
                <span class="self-biography">A free thinker roaming Earth.</span>
            </div>
        </div>
    </div>
    <div class="content">
        <div class="sidebar">
        </div>
        <div class="contacts">
            {#each contacts as contact}
                <Contact
                        user={contact}
                        messageStore={messageStore}
                        hour="10:00am"
                        picture="/assets/no_profile_picture.jpg"
                        bind:selected={selectedContact}
                />
            {/each}
        </div>
        <main class="block">
            {#if selectedContact}
                {#key selectedContact.id.toString()}
                    <Chat
                            user={data.user}
                            bind:contact={selectedContact}
                            store={messageStore}
                            keyStore={keydownStore}
                            token={data.token}
                    />
                {/key}
            {:else}
                <h1 class="nothing">Select a contact to start chatting.</h1>
            {/if}
        </main>

    </div>
</div>

<style>
    .page {
        display: flex;
        flex-direction: column;
        width: 100vw;
        height: 100vh;
        color: #b1b1b1;
        background-color: var(--background);
        overflow: hidden;
    }
    .header {
        display: flex;
        align-items: center;
        min-height: 55px;
        width: 100%;
        background-color: var(--light-contrast);
        border-bottom: 1px solid var(--thin-border);
    }
    .icon-space {
        height: 100%;
        width: 60px;
        border-right: 1px solid var(--thin-border);
    }
    .self-profile {
        display: flex;
        align-items: center;
        min-width: 200px;
        margin-right: 30px;
        padding: 0 14px;
        font-family: 'DM Sans', sans-serif;
        gap: 6px;
        user-select: none;
        cursor: pointer;
    }
    .self-profile:hover {
        background-color: var(--heavy-constrast);
    }
    .self-name {
        color: var(--text-color);
        font-weight: 600;
    }
    .self-username {
        font-size: 12px;
        color: var(--blurred-username);
    }
    .self-biography {
        font-size: 12px;
        color: var(--biography-contact);
    }
    .self-identifier {
        display: flex;
        flex-direction: column;
    }
    .photo {
        width: 40px;
        height: 40px;
        border-radius: 50%;
        margin: 5px;
        object-fit: cover;
    }
    .content {
        display: flex;
        width: 100%;
        height: calc(100% - 55px);
    }
    .sidebar {
        min-width: 60px;
        background-color: var(--light-contrast);
        border-right: 1px solid var(--thin-border);
    }
    .contacts {
        padding-top: 24px;
        display: flex;
        flex-direction: column;
        align-items: flex-end;
        width: 600px;
        gap: 24px;
    }
    .block {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 100%;
    }
    .nothing {
        font-family: 'DM Sans', sans-serif;
        font-size: 32px;
    }
    .theme-toggle-container {
        margin-left: auto;
        margin-right: 14px;
    }
</style>