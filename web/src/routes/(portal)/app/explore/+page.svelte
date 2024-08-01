<script>
    import PortalLayout from "../PortalLayout.svelte";
    import {getChatWithUser, globalSearch} from "$lib/network/api.ts";
    import {onMount} from "svelte";

    export let data;

    let searchTerm = '';
    let hasSearched = false;
    let hasResults = false;
    let results = [];

    let searchBox;

    onMount(() => {
        searchBox = document.getElementById('search-box-input');
        document.body.addEventListener('keydown', (event) => {
            let isInput = document.activeElement.tagName === 'INPUT';
            let isSearchInput = document.activeElement === searchBox;
            let isTypingInAnotherInput = isInput && !isSearchInput;
            if (isTypingInAnotherInput) return;
            if (event.key === 'Enter') {
                search();
                return;
            }
            if (!isSearchInput) {
                searchBox.focus();
            }
        });
    });

    function search() {
        if (searchTerm.trim() === '') return;
        results = []
        hasSearched = true;
        hasResults = false;
        globalSearch(data.token, searchTerm).then((finalResults) => {
            hasResults = true;
            results = finalResults;
        });
    }

    function contactUser(userId) {
        console.log(`Contacting user with id ${userId}`);
        getChatWithUser(data.token, userId).then((response) => {
            if (response.ok) {
                response.json().then((data) => {
                    const channelId = data.channel_id;
                    window.location.href = `/app/chat/${channelId}`;
                });
            } else {
                console.error(`Failed to contact user with id ${userId}`);
            }
        });
    }
</script>

<PortalLayout>
    <div class="header-content" slot="header">
    </div>
    <div class="page">
        <div class="content">
            <h1 class="title">Explore Iris</h1>
            <div class="search-box-container">
                <input
                        id="search-box-input"
                        class="search-box"
                        placeholder="Search and discover new people as you wander through the Iris network!"
                        bind:value={searchTerm}
                />
                <button class="search-button" on:click={search}>
                    <i class="fa-solid fa-search"></i>
                </button>
            </div>
            {#if hasSearched}
                <div class="search-feedback">
                {#if !hasResults}
                    <p>Searching...</p>
                {:else if results.length === 0}
                    <p>No results found.</p>
                {:else}
                    <div class="results-container">
                        <h3 class="separator">USERS</h3>
                        <div class="results">
                            {#each results.users as user}
                                <div class="user result">
                                    <img class="user-avatar" src="/assets/no_profile_picture.jpg" alt="User's profile picture"/>
                                    <div class="user-info">
                                        <span class="user-name">{user.name}</span>
                                        <span class="user-username">@{user.username}</span>
                                    </div>
                                    <div class="actions">
                                        <button class="action-button" on:click={() => contactUser(user.id)}>
                                            <i class="fa-regular fa-envelope"></i>
                                        </button>
                                    </div>
                                </div>
                            {/each}
                        </div>
                    </div>
                {/if}
                </div>
            {/if}
        </div>
    </div>
</PortalLayout>

<svelte:head>
    <link rel="preload" href="/assets/fonts/MonaSans%5Bslnt,wdth,wght%5D.woff2" as="font" type="font/woff2" crossorigin="anonymous">
</svelte:head>

<style>
    @font-face {
        font-family: 'Mona Sans';
        src: url('/assets/fonts/MonaSans[slnt,wdth,wght].woff2') format('woff2');
        font-stretch: extra-expanded;
    }

    .header-content {
        width: 100%;
    }
    .page {
        display: flex;
        justify-content: center;
        width: 100%;
        height: 100%;
        background-color: var(--background);

    }
    .content {
        display: flex;
        align-items: center;
        flex-direction: column;
        width: 80%;
        height: 100%;
    }
    .title {
        font-size: 3.5rem;
        font-weight: 700;
        font-family: 'Mona Sans', sans-serif;
        color: var(--text-color);
        margin-top: 3rem;
    }
    .search-box-container {
        display: flex;
        align-items: center;
        height: 56px;
        width: 80%;
        background-color: var(--light-contrast);
        border-radius: 512px;
        box-shadow: 0 24px 50px var(--heavy-constrast);
        font-family: 'Roboto', sans-serif;
    }
    .search-box {
        border: none;
        background-color: transparent;
        width: 100%;
        outline: none;
        color: var(--text-color);
        font-size: 0.95rem;
        padding-left: 28px;
    }
    .search-box::placeholder {
        color: var(--icon-selected-contrast);
    }
    .search-button {
        border: none;
        color: var(--text-color);
        font-size: 1.2rem;
        border-radius: 50%;
        height: 80%;
        aspect-ratio: 1/1;
        background-color: var(--search-button);
        cursor: pointer;
        margin-right: 10px;
        transition: background-color 125ms ease-in-out;
    }
    .search-button:hover {
        background-color: var(--search-button-hover);
    }
    .search-feedback {
        margin-top: 48px;
        width: 70%;
    }
    .results-container {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        justify-content: flex-start;
        width: 100%;
        font-family: 'Roboto', sans-serif;
    }
    .separator {
        font-size: 1.1rem;
        font-weight: 700;
        color: var(--reply-color);
        margin-bottom: 12px;
    }
    .results {
        display: flex;
        flex-direction: column;
        width: 100%;
        gap: 16px;
    }
    .user {
        width: 100%;
        height: 64px;
        background-color: var(--light-contrast);
        display: flex;
        align-items: center;
        border-radius: 8px;
        gap: 12px;
    }
    .user-avatar {
        margin-left: 16px;
        height: 44px;
        aspect-ratio: 1/1;
        border-radius: 50%;
    }
    .user-info {
        display: flex;
        align-items: center;
        gap: 8px;
    }
    .user-name {
        font-size: 1.1rem;
        font-weight: 700;
        color: var(--text-color);
    }
    .user-username {
        font-size: 0.9rem;
        font-weight: 400;
        color: var(--icon-selected-contrast);
    }
    .actions {
        display: flex;
        margin-left: auto;
        margin-right: 24px;
    }
    .action-button {
        height: 44px;
        aspect-ratio: 1/1;
        font-size: 1.5rem;
        background-color: transparent;
        border: 1px solid var(--text-color);
        color: var(--text-color);
        border-radius: 8px;
        cursor: pointer;
        transition: background-color 275ms ease-in-out, color 275ms ease-in-out;
    }
    .action-button:hover {
        background-color: var(--text-color);
        color: var(--background);
    }
</style>