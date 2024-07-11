<script lang="ts">
    import {API} from "../../interaction/server.ts";

    async function submit() {
        const urlParams = new URLSearchParams(window.location.search);
        const redirect = urlParams.get('redirect');
        const username = document.getElementById('username').value;
        const password = document.getElementById('password').value;

        const response = await fetch(`${API}/login`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ identifier: username, password })
        });

        const result = document.getElementById('result');
        if (response.ok) {
            const data = await response.json();
            document.cookie = `token=${data.token}`;
            window.location.href = '/app';
        } else {
            result.innerText = 'Login failed';
        }
    }
</script>

<input type="text" name="username" id="username" placeholder="Username" required>
<input type="password" name="password" id="password" placeholder="Password" required>

<button type="submit" on:click={() => submit()}>Login</button>
<p id="result"></p>