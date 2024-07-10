<script lang="ts">
    import Contact from "$lib/components/Contact.svelte";
    import {onMount} from "svelte";
    import {DOMAIN} from "../../../interaction/server.ts";

    export let data;
    let contacts: any[] = [];

    onMount(() => {
        fetch(`http://${DOMAIN}/api/contacts/@me`, {
            method: 'GET',
            headers: {
                'Authorization': 'Bearer ' + data.token
            }
        }).then(response => response.json()).then(contact_list => {
            contacts = contact_list;
        });
    })
</script>

<div class="page">
    <div class="contacts">
        <div class="self">

        </div>
        {#each contacts as contact}
            <Contact name={contact.name} text="Start a conversation" hour="10:00am" picture="/assets/no_profile_picture.jpg"/>
        {/each}
    </div>
    <main class="block">
        <div class="current-chat">
            <h1>{data.user.name}</h1>
        </div>
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