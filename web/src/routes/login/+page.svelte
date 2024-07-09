<script lang="ts">
    import {DOMAIN} from "../../interaction/server.ts";

    async function submit() {
        const username = document.getElementById('username').value;
        const password = document.getElementById('password').value;

        const response = await fetch(`http://${DOMAIN}/login`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ identifier: username, password })
        });

        const result = document.getElementById('result');
        if (response.ok) {
            const data = await response.json();
            result.innerText = `Login successful, you are ${data.user.name} (@${data.user.username}), with ID ${data.user.id} and email ${data.user.email}, your token is ${data.token}`;
        } else {
            result.innerText = 'Login failed';
        }
    }
</script>

<input type="text" name="username" id="username" placeholder="Username" required>
<input type="password" name="password" id="password" placeholder="Password" required>

<button type="submit" on:click={() => submit()}>Login</button>
<p id="result"></p>