<script lang="ts">
    import ThemeToggle from "$lib/components/ThemeToggle.svelte";

    export let data;
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
        <div class="container">
            <slot></slot>
        </div>
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
        min-height: 60px;
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
        height: 100%;
    }
    .sidebar {
        min-width: 60px;
        background-color: var(--light-contrast);
        border-right: 1px solid var(--thin-border);
    }
    .container {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
        height: 100%;
    }
    .theme-toggle-container {
        margin-left: auto;
        margin-right: 14px;
    }
</style>