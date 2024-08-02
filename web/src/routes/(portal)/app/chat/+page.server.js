import {fetchContacts} from "$lib/network/api.ts";

export async function load({ url, parent }) {
    const { token } = await parent();
    const contactId = url.searchParams.get('c');
    const contacts = await fetchContacts(token);
    return {
        contacts,
        requestedContactChannelId: parseInt(contactId),
    };
}