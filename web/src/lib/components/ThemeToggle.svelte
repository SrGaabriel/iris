<script>
    import {onMount} from "svelte";

    let isLight = false;

    onMount(() => {
        const theme = localStorage.getItem('theme');
        if (theme === 'light') {
            document.body.classList.add('lightmode');
            isLight = true;
        }
    });

    function toggleTheme() {
        if (isLight) {
            localStorage.setItem('theme', 'dark');
            document.body.classList.remove('lightmode');
        } else {
            localStorage.setItem('theme', 'light');
            document.body.classList.add('lightmode');
        }
        isLight = !isLight;
    }
</script>

<button class="toggle {isLight ? 'light' : 'dark'}" on:click={() => toggleTheme()}>
    <i class="fa-regular fa-sun icon {isLight ? 'enabled' : 'disabled'}"></i>
    <i class="fa-regular fa-moon icon {isLight ? 'disabled' : 'enabled'}"></i>
    <div class="enable-bubble" style="translate: {isLight ? '-' : '+'}60%"></div>
</button>

<style>
    .toggle {
        position: relative;
        display: flex;
        align-items: center;
        justify-content: space-around;
        gap: 24px;
        width: 80px;
        height: 36px;
        border-radius: 32px;
        background-color: transparent;
        border: 2px solid var(--theme-toggle-contrast);
        overflow: hidden;
        cursor: pointer;
        user-select: none;
    }
    .icon {
        position: relative;
        z-index: 2;
        font-size: 21px;
        transition: color 0.5s;
    }
    .fa-sun.enabled {
        color: #f1c40f;
    }
    .fa-moon.enabled {
        color: #0f045d;
    }
    .fa-sun.disabled {
        color: white;
    }
    .fa-moon.disabled {
        color: black;
    }
    .enable-bubble {
        position: absolute;
        height: calc(100% + 4px);
        background-color: var(--theme-toggle-contrast);
        border-radius: inherit;
        z-index: 1;
        aspect-ratio: 1/1;
        transition: translate 0.5s;
    }
</style>