<script lang="ts">
    import Contact from "$lib/components/Contact.svelte";
    import {onMount} from "svelte";
    import server, {API} from "../../../interaction/server.ts";
    import Chat from "$lib/components/Chat.svelte";
    import {writable} from "svelte/store";
    import Nothing from "$lib/components/Nothing.svelte";

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
    <div class="header">
    </div>
    <div class="content">
        <div class="sidebar">

        </div>
        <div class="contacts">
            {#each contacts as contact}
                <Contact user={contact} text="These are the things I want you to take care of. Please take a good look at them." hour="10:00am" picture="/assets/no_profile_picture.jpg" bind:selected={selectedContact}/>
            {/each}
        </div>
        <main class="block">
            {#if selectedContact}
                {#key selectedContact.id.toString()}
                    <Chat user={data.user} bind:contact={selectedContact} store={messageStore} token={data.token}/>
                {/key}
            {:else}
                <Nothing/>
            {/if}
        </main>
        {#if selectedContact}
            <div class="specific-contact-container">
                <div class="specific-contact">

                </div>
            </div>
        {/if}
    </div>
</div>

<style>
    .page {
        display: flex;
        flex-direction: column;
        width: 100vw;
        height: 100vh;
        color: #646464;
        background-color: #141414;
        overflow: hidden;
    }
    .header {
        min-height: 50px;
        width: 100%;
        background-color: #2a2a2a;
        border-bottom: 1px solid #646464;
    }
    .content {
        display: flex;
        width: 100%;
        height: calc(100% - 50px);
    }
    .sidebar {
        min-width: 60px;
        border-top: 1px solid #2a2a2a;
        background-color: #2a2a2a;
        border-right: 1px solid #646464;
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
    .specific-contact-container {
        display: flex;
        align-items: center;
        height: 100%;
        width: 600px;
    }
    .specific-contact {
        background-color: #2a2a2a;
        border-radius: 12px;
        margin-left: auto;
        width: 100%;
        height: 95%;
    }
</style>