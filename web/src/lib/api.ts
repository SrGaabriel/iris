import { API } from "../interaction/server.ts";

export async function fetchContacts(token: string) {
    const response = await fetch(`${API}/api/contacts/@me`, {
        method: 'GET',
        headers: {
            'Authorization': `Bearer ${token}`
        }
    });
    const text = await response.text();
    return JSONbig.parse(text);
}

export async function fetchMessages(token: string, contactId) {
    const response = await fetch(`${API}/api/messages/${contactId}`, {
        method: 'GET',
        headers: {
            'Authorization': `Bearer ${token}`
        }
    });
    const text = await response.text();
    return JSONbig.parse(text);
}

export async function sendMessage(token: string, contactId, content: string) {
    const response = await fetch(`${API}/api/messages/${contactId}`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ content })
    });
    const text = await response.text();
    return JSONbig.parse(text);
}
