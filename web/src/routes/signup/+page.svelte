<script lang="ts">
    import {API} from "$lib/network/server.ts";

    async function submit() {
        const name = (document.getElementById('name') as HTMLInputElement).value;
        const email = (document.getElementById('email') as HTMLInputElement).value;
        const username = (document.getElementById('username') as HTMLInputElement).value;
        const password = (document.getElementById('password') as HTMLInputElement).value;

        const response = await fetch(`${API}/signup`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({name, email, username, password})
        });

        const result = await response.json();
        const resultMessage = document.getElementById('result')!;
        if (result.success) {
            resultMessage.innerHTML = "Success! You can now login."
        } else {
            resultMessage.innerHTML = result.message;
        }
    }
</script>

<main>

    <input type="text" name="name" id="name" placeholder="Name" required>
    <input type="text" name="email" id="email" placeholder="Email" required>
    <input type="text" name="username" id="username" placeholder="Username" required>
    <input type="password" name="password" id="password" placeholder="Password" required>

    <button type="submit" on:click={() => submit()}>Login</button>
    <p id="result"></p>
</main>

<style>
    main {
        display: flex;
        flex-direction: column;
        width: 300px;
    }
</style>