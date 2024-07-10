<script lang="ts">
    // we have two parameters for this component
    // profile picture, text, hour

    export let picture: string;
    export let name: string;
    export let text: string;
    export let hour: string;
</script>

<div class="contact">
    <img src={picture} alt={`${name}'s profile picture`} class="photo"/>
    <div class="text">
        <span class="name">{name}</span>
        <span class="message">{text}</span>
    </div>
    <span class="date">{hour}</span>
</div>

<style>
    .contact {
        font-family: 'DM Sans', sans-serif;
        display: flex;
        align-items: center;
        width: 100%;
        height: 75px;
        border-bottom: 1px solid #e9e9e9;
        transition: background-color 0.2s;
        user-select: none;
    }

    .contact:hover {
        background-color: #f9f9f9;
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
        flex-direction: column;
    }

    .name {
        font-size: 16px;
        font-weight: 600;
    }

    .message {
        font-weight: 400;
        color: #2e2e2e;
        font-size: 13px;
    }

    .date {
        font-size: 12px;
        color: gray;
        margin-bottom: auto;
        margin-left: auto;
        margin-top: 12px;
        margin-right: 8px;
    }
</style>