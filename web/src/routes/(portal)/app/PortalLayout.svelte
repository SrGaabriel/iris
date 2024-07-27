<!--This is a component, and not a layout page due to:-->
<!--https://github.com/sveltejs/kit/issues/627-->

<script lang="ts">
    import {browser} from "$app/environment";

    let isSidebarExpanded = false;

    // TODO: fix workaround
    function getActivePage() {
        if (!browser) return;
        const url = window.location.pathname;
        if (url.includes('/app/contacts')) {
            return 'contacts';
        } else if (url.includes('/app/chats')) {
            return 'chats';
        } else if (url.includes('/app/settings')) {
            return 'settings';
        }
    }
</script>

<div class="page">
    <div class="background"></div>
    <div class="space">
        <div class="header">
            <div class="icon-space"></div>
            <slot name="header"></slot>
        </div>
        <div class="content">
            <div class={isSidebarExpanded ? 'sidebar expanded' : 'sidebar'}>
                <button class="sidebar-button" data-active={getActivePage() === 'inbox'}>
                    <i class="fa-solid fa-inbox"></i>
                    <span class="button-label">Inbox</span>
                </button>
                <button class="sidebar-button" data-active={getActivePage() === 'contacts'}>
                    <i class="fa-regular fa-message"></i>
                    <span class="button-label">Contacts</span>
                </button>
                <button class="sidebar-button" data-active={getActivePage() === 'notes'}>
                    <i class="fa-regular fa-note-sticky"></i>
                    <span class="button-label">Notes</span>
                </button>
                <button class="sidebar-button" data-active={getActivePage() === 'settings'}>
                    <i class="fa-solid fa-sliders"></i>
                    <span class="button-label">Settings</span>
                </button>
            </div>
            <div class={isSidebarExpanded ? 'container expanded' : 'container'}>
                <slot></slot>
            </div>
        </div>
    </div>
</div>

<style>
    .page {
        position: relative;
        display: flex;
        flex-direction: column;
        width: 100vw;
        height: 100vh;
        color: #b1b1b1;
        background-color: var(--background);
        overflow: hidden;
    }
    .background {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: 0;
        transform: scale(1.4);
        filter: blur(40px);
        background: var(--light-contrast) no-repeat center center fixed;
    }
    .space {
        position: absolute;
        z-index: 1;
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
    }
    .header {
        display: flex;
        align-items: center;
        min-height: 60px;
        width: 100%;
        background-color: var(--light-heavier-contrast);
        border-bottom: 1px solid var(--thin-border);
    }
    .standard-header-info {
        display: flex;
        height: 100%;
        width: 399px;
        border-right: 1px solid var(--thin-border);
    }
    .icon-space {
        height: 100%;
        width: 84.5px;
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
        position: relative;
        z-index: 10;
        display: flex;
        flex-direction: column;
        align-items: center;
        min-width: 80px;
        max-width: 80px;
        padding-top: 12px;
        gap: 12px;
        background-color: var(--light-contrast-opaque);
        transition: all 0.2s;
        overflow-x: hidden;
        border-right: 1px solid var(--thin-border);
    }
    .sidebar.expanded {
        gap: 24px;
        transform: scaleX(5);
    }
    .sidebar-button {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 75%;
        border: none;
        background-color: transparent;
        font-family: 'Roboto', sans-serif;
        gap: 8px;
        color: var(--icon-color);
        font-size: 20px;
        border-radius: 12px;
        aspect-ratio: 1/1;
        cursor: pointer;
        transition: all 125ms ease-in-out;
    }
    .sidebar-button[data-active="false"]:hover {
        background-color: var(--icon-selected);
        color: var(--icon-selected-contrast);
        opacity: 0.6;
    }
    .sidebar.expanded .sidebar-button {
        justify-content: flex-start;
        flex-direction: row;
        border-radius: 12px;
        background-color: red;
        transform: scaleX(0.2);
        font-size: 24px;
        gap: 8px;
        aspect-ratio: auto;
        width: 280%;
        margin-left: 40%;
        transform-origin: center;
    }
    .sidebar.expanded .button-label {
        font-size: 12px;
    }
    .container.expanded {
        transform: translateX(120px);
    }
    .button-label {
        font-size: 11px;
    }
    .sidebar-button[data-active="true"] {
        color: var(--icon-selected-contrast);
        background-color: var(--icon-selected);
        cursor: default;
        font-weight: 500;
    }
    .container {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
        height: 100%;
        transition: all 0.2s;
    }
    .theme-toggle-container {
        margin-left: auto;
        margin-right: 14px;
    }
</style>