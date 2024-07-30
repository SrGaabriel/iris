import {API} from "./server.ts";

export async function fetchContacts(token: string) {
    const response = await fetch(`${API}/api/contacts/@me`, {
        method: 'GET',
        headers: {
            'Authorization': `Bearer ${token}`
        }
    });
    const text = await response.text();
    return JSON.parse(text);
}

export async function fetchContact(token: string, contactId) {
    const response = await fetch(`${API}/api/contacts/${contactId}`, {
        method: 'GET',
        headers: {
            'Authorization': `Bearer ${token}`
        }
    });
    const text = await response.text();
    return JSON.parse(text);
}

export async function fetchMessages(token: string, channelId) {
    const response = await fetch(`${API}/api/channels/${channelId}/messages`, {
        method: 'GET',
        headers: {
            'Authorization': `Bearer ${token}`
        }
    });
    const text = await response.text();
    return JSON.parse(text);
}

export async function sendMessage(token: string, channelId, content: string, replyingTo) {
    const response = await fetch(`${API}/api/channels/${channelId}/messages`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            content,
            reply_to: replyingTo
        })
    });
    const text = await response.text();
    return JSON.parse(text);
}

// Submit edit
export async function editMessage(token: string, channelId, messageId, content: string) {
    const response = await fetch(`${API}/api/channels/${channelId}/messages/${messageId}`, {
        method: 'PUT',
        headers: {
            'Authorization': `Bearer ${token}`,
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            content
        })
    });
    const text = await response.text();
    return JSON.parse(text);
}

export async function excludeMessage(token: string, channelId, messageId) {
    return await fetch(`${API}/api/channels/${channelId}/messages/${messageId}`, {
        method: 'DELETE',
        headers: {
            'Authorization': `Bearer ${token}`
        }
    });
}

export async function addReaction(token: string, channelId, messageId, emoji) {
    const response = await fetch(`${API}/api/channels/${channelId}/messages/${messageId}/reactions`, {
        method: 'POST',
        headers: {
            'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify({
            reaction_type: emoji
        })
    });
    return response.json()
}

export async function deleteReaction(token: string, channelId, messageId, reactionId) {
    return await fetch(`${API}/api/channels/${channelId}/messages/${messageId}/reactions/${reactionId}`, {
        method: 'DELETE',
        headers: {
            'Authorization': `Bearer ${token}`
        }
    });
}