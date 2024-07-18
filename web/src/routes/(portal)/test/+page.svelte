<script>
    import EmojiMenu from "$lib/components/EmojiMenu.svelte";
</script>

<div class="page">
    <EmojiMenu onClick={(emoji) => alert(`${emoji.emoji}, ${emoji.slug}`)}/>
</div>

<style>
    .page {
        width: 100vw;
        height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
    }
</style>