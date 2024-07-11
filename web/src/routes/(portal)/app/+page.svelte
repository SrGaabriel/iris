<script lang="ts">
    import Contact from "$lib/components/Contact.svelte";
    import {onMount} from "svelte";
    import server, {API} from "../../../interaction/server.ts";
    import Chat from "$lib/components/Chat.svelte";
    import {writable} from "svelte/store";

    export let data;
    let contacts: any[] = [];
    $: selectedContact = null;
    let messageStore = writable();

    onMount(() => {
        fetch(`${API}/api/contacts/@me`, {
            method: 'GET',
            headers: {
                'Authorization': 'Bearer ' + data.token
            }
        }).then(response => response.text()).then((text) => JSONbig.parse(text)).then(contact_list => {
            contacts = contact_list;
        });
        server.connect(data.token);
        server.subscribe((message) => {
            messageStore.set(message);
        });
    })
</script>

<div class="page">
    <div class="contacts">
        <div class="self">
            <p>{data.user.name}</p>
        </div>
        {#each contacts as contact}
            <Contact user={contact} text="Start a conversation" hour="10:00am" picture="/assets/no_profile_picture.jpg" bind:selected={selectedContact}/>
        {/each}
    </div>
    <main class="block">
        {#if selectedContact}
            {#key selectedContact.id.toString()}
                <Chat user={data.user} bind:contact={selectedContact} store={messageStore} token={data.token}/>
            {/key}
        {/if}
    </main>
</div>

<style>
    .page {
        display: flex;
        flex-direction: row;
        width: 100vw;
        height: 100vh;
        background-color: #eaeaea;
    }
    .contacts {
        display: flex;
        flex-direction: column;
        width: 25%;
        background-color: #ffffff;
        border-right: 2px solid #e9e9e9;
    }
    .self {
        width: 100%;
        height: 75px;
        background-color: #f0f0f0;
        border-bottom: 2px solid #e9e9e9;
    }
    .block {
        display: flex;
        flex-direction: column;
        width: 100%;
    }
    .current-chat {
        background-color: #ffffff;
        border-bottom: 2px solid #e9e9e9;
        width: 100%;
        height: 75px;
    }
</style>