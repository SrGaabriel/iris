<script lang="ts">
    // we have two parameters for this component
    // profile picture, text, hour

    export let picture: string;
    export let user: any;
    export let username: string;
    export let text: string;
    export let hour: string;
    export let selected: any;
</script>

<button class="contact" on:click={() => selected = user}>
    <div class="top">
        <img src={picture} alt={`${user.name}'s profile picture`} class="photo"/>
        <div class="text">
            <div class="identifier">
                <span class="name">{user.name}</span>
                <span class="username">@{user.username}</span>
            </div>
            <span class="biography">A free thinker roaming Earth.</span>
        </div>
    </div>
    <div class="last-message-container">
        <span class="last-message">{text}</span>
    </div>
</button>

<style>
    .contact {
        font-family: 'DM Sans', sans-serif;
        display: flex;
        flex-direction: column;
        width: 95%;
        min-height: 75px;
        border: none;
        outline: none;
        border-radius: 12px;
        background-color: #2a2a2a;
        transition: all 0.2s;
        user-select: none;
        cursor: pointer;
        padding: 10px 4px;
    }

    .top {
        display: flex;
        align-items: center;
    }

    .contact:hover {
        transform: translateY(-5px);
        background-color: #242424;
    }

    .photo {
        width: 50px;
        height: 50px;
        border-radius: 50%;
        margin: 10px;
        object-fit: cover;
    }

    .text {
        display: flex;
        align-items: flex-start;
        flex-direction: column;
    }

    .name {
        font-size: 16px;
        font-weight: 600;
        color: white;
    }

    .identifier {
        display: flex;
        align-items: center;
        gap: 4px;
    }

    .username {
        font-weight: 400;
        color: #626262;
        font-size: 13px;
    }

    .biography {
        font-weight: 400;
        color: #929292;
        font-size: 13px;
    }

    .last-message-container {
        display: flex;
        justify-content: flex-start;
        margin: 6px 10px;
    }

    .last-message {
        text-align: left;
        color: white;
    }

    .date {
        font-size: 12px;
        color: #6e6e6e;
        margin-bottom: auto;
        margin-left: auto;
        margin-top: 12px;
        margin-right: 8px;
    }
</style>