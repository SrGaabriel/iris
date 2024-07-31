<script>
    import Contact from "$lib/components/Contact.svelte";
    import {onMount} from "svelte";
    import server from "$lib/network/server.ts";
    import {writable} from "svelte/store";
    import TargetedStore from '../../../../util/targetedStore.ts';
    import {
        CONTACT_TYPING_ID,
        CHANNEL_READ_ID,
        MESSAGE_CREATED_ID
    } from "$lib/network/message.ts";
    import {TYPING_DELAY} from "$lib/constants.ts";
    import {fetchContacts} from "$lib/network/api.ts"
    import Chat from "$lib/components/Chat.svelte";
    import PortalLayout from "../PortalLayout.svelte";
    import {pushState} from "$app/navigation";

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
            if (data.requestedContactChannelId) {
                let foundContact = contact_list.find(contact =>
                    contact.channel_id === parseInt(data.requestedContactChannelId)
                );
                if (foundContact) {
                    selectedContact = foundContact;
                    pushState(`/app/chat/${data.requestedContactChannelId}`, null);
                }
            }
        });
    }

    function onMessageCreated(create) {
        const message = create.message;
        messageStore.set(message);
        const channelTimers = typing[message.channel_id];
        if (!channelTimers) return;
        typing[message.channel_id] = channelTimers.filter((typing) => {
            if (typing.user.id === message.user_id) {
                clearTimeout(typing.timeout);
                return false;
            }
            return true;
        });
        typing = typing;
    }

    function onTyping(message) {
        if (!message) return;
        const previousTyping = typing[message.channel_id];
        if (previousTyping) {
            previousTyping.filter((typing) => {
                if (typing.user.id === message.user.id) {
                    clearTimeout(typing.timeout);
                    return false;
                }
                return true;
            });
        } else {
            typing[message.channel_id] = [];
        }

        typing[message.channel_id].push({
            user: message.user,
            timeout: setTimeout(() => {
                typing[message.channel_id] = typing[message.channel_id].filter((typing) => {
                    return typing.user.id !== message.user.id;
                });
                typing = typing;
            }, TYPING_DELAY)
        });
        console.log('Typing: ', typing);
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
                                typing={(typing[contact.channel_id] ?? []).some((typing) => {
                                    return typing.user.id === contact.user_id;
                                })}
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
                        channelId={selectedContact.channel_id}
                        typingUsers={(typing[selectedContact.channel_id] ?? []).map((typing) => {
                            console.log(typing);
                            return typing.user;
                        })}
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