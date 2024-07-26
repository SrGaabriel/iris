<script>
    import Contact from "$lib/components/Contact.svelte";
    import {onMount} from "svelte";
    import server from "$lib/network/server.ts";
    import {writable} from "svelte/store";
    import TargetedStore from '../../../../util/targetedStore.ts';
    import {
        CONTACT_TYPING_ID,
        CONTEXT_READ_ID,
        MESSAGE_CREATED_ID
    } from "$lib/network/message.ts";
    import {TYPING_DELAY} from "$lib/constants.ts";
    import {fetchContacts} from "$lib/network/api.ts"
    import Chat from "$lib/components/Chat.svelte";

    export let data;
    let contacts = [];
    let messageStore = writable();
    let keydownStore = new TargetedStore();
    $: typing = {};
    $: amITyping = false;
    $: selectedContact = null;

    onMount(() => {
        loadContacts();

        server.connect(data.token);
        server.store.subscribe(MESSAGE_CREATED_ID, onMessageCreated);
        server.store.subscribe(CONTACT_TYPING_ID, onTyping);
    });

    function loadContacts() {
        fetchContacts(data.token).then(contact_list => {
            contacts = contact_list;
            return contact_list;
        }).then((contact_list) => {
            if (data.requestedContactId) {
                let foundContact = contact_list.find(contact =>
                    contact.id === parseInt(data.requestedContactId)
                );
                if (foundContact) {
                    selectedContact = foundContact;
                    history.pushState({}, '', `/app/contacts/${data.requestedContactId}`);
                }
            }
        });
    }

    function onMessageCreated(message) {
        messageStore.set(message);
        if (message.userId === data.user.id)
            return;
        const previousTimeout = typing[message.userId];
        if (previousTimeout) {
            delete typing[message.userId];
            clearTimeout(previousTimeout);
        }
        if (selectedContact && message.userId === selectedContact.id) {
            server.sendPacket(CONTEXT_READ_ID, {contextId: selectedContact.id});
        }
        typing = typing;
    }

    function onTyping(message) {
        const previousTimeout = typing[message.contactId];
        if (previousTimeout) {
            clearTimeout(previousTimeout);
        }

        typing[message.contactId] = setTimeout(() => {
            delete typing[message.contactId];
            typing = typing;
        }, TYPING_DELAY);
        typing = typing;
    }
</script>

<div class="page">
    <div class="contacts">
        {#each contacts as contact}
            <Contact
                    selfId={data.user.id}
                    user={contact}
                    messageStore={messageStore}
                    picture="/assets/no_profile_picture.jpg"
                    bind:selected={selectedContact}
                    typing={!!typing[contact.id]}
            />
        {/each}
    </div>
    <main class="block">
        {#if selectedContact}
            <Chat
                token={data.token}
                user={data.user}
                contact={selectedContact}
            />
        {/if}
    </main>
</div>

<style>
    .page {
        display: flex;
        width: 100vw;
        height: 100vh;
        color: #b1b1b1;
        background-color: var(--background);
        overflow: hidden;
    }
    .contacts {
        padding-top: 24px;
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 550px;
        gap: 24px;
    }
    .block {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        margin-top: 24px;
        width: 100%;
        padding: 0;
        height: 95%;
    }
</style>