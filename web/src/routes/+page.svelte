<script lang="ts">
    import {onMount} from "svelte";
    import server from "../interaction/server.ts"

    let text = '';
    let messages: string[] = [];

    function sendMessage() {
        server.sendMessage(text);
        messages = [...messages, text];
        text = '';
    }

    onMount(() => {
        server.connect();
        server.subscribe((currentMessage) => {
            messages = [...messages, currentMessage];
        });
    });
</script>

<input type="text" bind:value={text}/>
<button on:click={() => sendMessage()}>Send</button>
    {#each messages as message}
        <p>{message}</p>
    {/each}