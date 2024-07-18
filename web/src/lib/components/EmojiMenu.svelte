<script lang="ts">
    import inefficientData from '$lib/data/emojis.json';
    import {onMount} from "svelte";
    import VirtualList from '@sveltejs/svelte-virtual-list';

    const jsonData = createReducedData();
    let currentCategory = jsonData[0].slug;
    let loadedCategories = new Set();
    let searchTerm = '';

    export let onClick;

    export let bottom = 0;
    export let top = 0;
    export let left = 0;
    export let right = 0;
    export let center = false;

    function createReducedData() {
        return inefficientData.map(category => {
            return {
                name: category.name,
                slug: category.slug,
                emojis: category.emojis.map(emoji => {
                    return {
                        name: emoji.name,
                        emoji: emoji.emoji,
                        slug: emoji.slug
                    };
                })
            };
        });
    }

    function getCategoryEmojis(loaded, term, category) {
        if (!term || term === '') {
            return category.emojis;
        }
        return category.emojis.filter(emoji => {
            return emoji.name.toLowerCase().indexOf(term.toLowerCase()) >= 0;
        });
    }

    function getCategoryIcon(category) {
        const icons = {
            "smileys_emotion": "fa-regular fa-face-smile",
            "people_body": "fa-regular fa-eye",
            "animals_nature": "fa-solid fa-dog",
            "food_drink": "fa-regular fa-lemon",
            "travel_places": "fa-regular fa-compass",
            "activities": "fa-solid fa-person-running",
            "objects": "fa-regular fa-gem",
            "symbols": "fa-regular fa-heart",
            "flags": "fa-regular fa-flag",
        };
        return icons[category.slug] || console.error(`Unknown category: ${category.slug}`);
    }
    let emojisSection;

    onMount(() => {
        const search = document.getElementById('search-box')!;
        const categories = document.querySelectorAll('.emojis-category')!;
        emojisSection = document.getElementById('emojis-section')!;

        search.focus();

        const emojisSectionRect = emojisSection.getBoundingClientRect();
        const emojisSectionTop = emojisSectionRect.top;

        emojisSection.addEventListener('scroll', () => {
            let newCurrentCategory = currentCategory;

            for (let i = 0; i < categories.length; i++) {
                const category = categories[i];
                const rect = category.getBoundingClientRect();

                if (rect.bottom >= emojisSectionTop && rect.top <= emojisSectionTop) {
                    newCurrentCategory = category.dataset.slug;
                    break;
                }
            }

            currentCategory = newCurrentCategory;
        });

        // const observer = new IntersectionObserver((entries) => {
        //     entries.forEach(entry => {
        //         const slug = entry.target.dataset.slug;
        //         const isLoaded = loadedCategories.has(slug);
        //         if (entry.isIntersecting && !isLoaded) {
        //             console.log("Loading", slug);
        //             loadedCategories.add(slug);
        //             loadedCategories = loadedCategories;
        //         }
        //     });
        // });
        //
        // categories.forEach(categoryElement => {
        //     console.log("Observing");
        //     observer.observe(categoryElement);
        // });
    })

    function scrollToCategory(category) {
        const categoryElement = document.getElementById(category.slug);
        if (!categoryElement) return;

        if (!loadedCategories.has(category.slug)) {
            loadedCategories.add(category.slug);
            loadedCategories = loadedCategories;
        }

        setTimeout(() => {
            emojisSection.scrollTo({
                top: categoryElement.offsetTop - 90,
                behavior: 'smooth'
            });
        }, 30);
    }

    function createPositionString() {
        if (!center) {
            return `top: ${top}px; bottom: ${bottom}px; left: ${left}px; right: ${right}px`;
        }
        return `top: 50%; left: 50%; transform: translate(-50%, -50%)`;
    }
</script>

<div class="menu" style={createPositionString()}>
    <div class="header">
        <input
                id="search-box"
                type="text"
                class="search-box"
                placeholder="Search emojis"
                bind:value={searchTerm}
        />
        <div class="categories">
            {#each jsonData as category}
                {@const chosen = category.slug === currentCategory ? "chosen" : ""}
                <button class={`category-icon ${chosen}`} on:click={() => scrollToCategory(category)}>
                    <i class={getCategoryIcon(category)}></i>
                </button>
            {/each}
        </div>
    </div>
    <div class="emojis-section" id="emojis-section">
        {#each jsonData as category}
                <div class="emojis-category" data-slug={category.slug} id={category.slug}>
                    {#if true}
                        {@const emojis = getCategoryEmojis(loadedCategories, searchTerm, category)}
                        {#if !emojis || emojis.length > 0}
                            <span class="emoji-category-text" id={`category-text-${category.slug}`}>{category.name.toUpperCase()}</span>
                            <div class="emojis-container">
                                <div class="emojis">
                                    {#each emojis as emoji}
                                        <button class="emoji" on:click={() => onClick(emoji)}>
                                            {emoji.emoji}
                                        </button>
                                    {/each}
                                </div>
                            </div>
                        {/if}
                    {/if}
                </div>
        {/each}
    </div>
</div>

<style>
    .menu {
        position: absolute;
        z-index: 8;
        width: 350px;
        height: 400px;
        border-radius: 4px;
        padding: 8px;
        background-color: var(--light-contrast);
        cursor: default;
        overflow: hidden;
        margin-top: auto;
        box-shadow: 0 0 12px var(--background);
    }
    .header {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
        justify-content: space-between;
        padding: 8px 0 2px 0;
        border-bottom: 1px solid var(--thin-border);
    }
    .search-box {
        height: 32px;
        width: 96%;
        border-radius: 8px;
        padding-left: 12px;
        border: none;
        color: var(--text-color);
        outline: none;
        font-size: 13px;
        font-family: 'Roboto', sans-serif;
        background-color: var(--heavy-constrast);
    }
    .categories {
        width: 100%;
        display: flex;
        align-items: center;
        height: 40px;
        justify-content: space-around;
    }
    .category-icon {
        display: flex;
        justify-content: center;
        align-items: center;
        font-size: 16px;
        user-select: none;
        -moz-user-select: none;
        cursor: pointer;
        border: none;
        color: var(--text-color);
        background-color: transparent;
        transition: all 0.2s;
        border-radius: 100%;
        aspect-ratio: 1/1;
        height: 80%
    }
    .category-icon:hover {
        color: #85a5ff;
        background-color: rgba(145, 197, 210, 0.28);
    }
    .category-icon.chosen {
        color: #6a92ff;
    }
    .emojis-section {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 300px;
        overflow: scroll;
    }
    .emojis-category {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }
    .emoji-category-text {
        font-size: 14px;
        font-weight: 600;
        margin: 10px 0;
        margin-left: 5%;
        font-family: 'Roboto', sans-serif;
        color: var(--message-sender-name);
    }
    .emojis-container {
        display: flex;
        width: 100%;
        justify-content: center;
        align-items: center;
    }
    .emojis {
        display: flex;
        width: 95%;
        flex-wrap: wrap;
        justify-content: flex-start;
        gap: 4px;
    }
    .emoji {
        display: flex;
        justify-content: center;
        align-items: center;
        cursor: pointer;
        border-radius: 50%;
        padding: 8px;
        transition: all 0.2s;
        border: none;
        aspect-ratio: 1/1;
        background-color: transparent;
        font-size: 32px;
        width: calc(15%);
    }
    .empty {
        width: 100%;
        height: 300px;
    }
    .emoji:hover {
        background-color: var(--reply-color);
        transform: translateY(-2px);
        box-shadow: 0 0 6px var(--text-color);
    }
</style>