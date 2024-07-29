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
    import PortalLayout from "../PortalLayout.svelte";

    export let data;
    let contacts = [];
    let messageStore = writable();
    let keydownStore = new TargetedStore();
    $: typing = {};
    $: amITyping = false;
    $: selectedContact = null;
    let contactSearch = '';

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

    function searchContacts(contacts, term) {
        let trimmed = term.trim();
        if (!term || trimmed === '') {
            return contacts;
        }
        return contacts.filter(contact => {
            return contact.name.toLowerCase().includes(trimmed.toLowerCase());
        });
    }
</script>

<PortalLayout>
    <div class="header-content" slot="header">
        <div class="header-contacts-space">
            <input
                    class="search-box"
                    type="text"
                    placeholder="Search contacts"
                    bind:value={contactSearch}
            />
        </div>
        <div class="header-contact-info">
            {#if selectedContact}
                <div class="header-contact-identifier">
                    <span class="header-contact-name">{selectedContact.name}</span>
                </div>
                <div class="header-contact-buttons">
                    <button class="header-contact-button">
                        <i class="fa-solid fa-phone"></i>
                    </button>
                    <button class="header-contact-button">
                        <i class="fa-solid fa-video"></i>
                    </button>
                    <button class="header-contact-button">
                        <i class="fa-solid fa-ellipsis"></i>
                    </button>
                </div>
            {/if}

        </div>
    </div>

    <div class="page">
        <div class="contacts">
<!--            Yes, this is necessary:-->
            {#if true}
                {@const contactList = searchContacts(contacts, contactSearch)}
                {#if contactList.length === 0}
                    <p class="no-contacts-found">No contacts found</p>
                {:else}
                    {#each contactList as contact}
                        <Contact
                                selfId={data.user.id}
                                user={contact}
                                messageStore={messageStore}
                                picture="/assets/no_profile_picture.jpg"
                                bind:selected={selectedContact}
                                typing={!!typing[contact.id]}
                        />
                    {/each}
                {/if}
            {/if}
        </div>
        <main class="block">
            {#if selectedContact}
                {#key selectedContact.id}
                    <Chat
                        token={data.token}
                        user={data.user}
                        contact={selectedContact}
                        contactTyping={!!typing[selectedContact.id]}
                    />
                {/key}
            {/if}
        </main>
    </div>
</PortalLayout>

<style>
    .page {
        display: flex;
        width: 100%;
        height: 100%;
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
    .no-contacts-found {
        font-family: 'Roboto', sans-serif;
    }
    .block {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        width: 100%;
        padding: 0;
        height: 100%;
    }
    .header-content {
        display: flex;
        width: 100%;
        height: 100%;
    }
    .header-contacts-space {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 550px;
        border-right: 1px solid var(--thin-border);
    }
    .search-box {
        width: 80%;
        height: 60%;
        color: var(--text-color);
        padding-left: 18px;
        background-color: var(--background);
        border: none;
        font-family: 'DM Sans', sans-serif;
        font-size: 14px;
        border-radius: 12px;
        outline: none;
    }
    .header-contact-info {
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 100%;
    }
    .header-contact-name {
        font-size: 18px;
        font-weight: 700;
        color: var(--text-color);
        font-family: 'DM Sans', sans-serif;
        margin-left: 24px;
    }
    .header-contact-buttons {
        display: flex;
        align-items: center;
        justify-content: flex-end;
        gap: 28px;
        margin-right: 36px;
        height: 100%;
    }
    .header-contact-button {
        background-color: var(--light-contrast);
        color: var(--text-color);
        font-size: 20px;
        height: 75%;
        aspect-ratio: 1/1;
        border-radius: 14px;
        border: 1px solid var(--icon-color);
        cursor: pointer;
        transition: background-color 0.2s, color 0.2s;
    }
    .header-contact-button:hover {
        background-color: var(--icon-selected);
        color: var(--icon-selected-contrast);
    }
</style>