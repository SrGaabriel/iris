<script>
    import Contact from "$lib/components/Contact.svelte";

    /** @type {import('./$types').LayoutData} */
    export let data;
</script>

<div class="page">
    <div class="contacts">
        <div class="self">

        </div>
        <Contact name="Sabrina" text="When are you coming over?" hour="9:56pm" picture="/assets/no_profile_picture.jpg"/>
        <Contact name="John" text="I'm on my way" hour="9:57pm" picture="/assets/no_profile_picture.jpg"/>
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