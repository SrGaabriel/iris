import {fetchContact, fetchMessages} from "$lib/network/api.ts";

export async function load({ params, parent }) {
    const { token } = await parent();
    const contact = await fetchContact(token, params.contact_id);
    console.log(params.contact_id);
    console.log(contact);
    const messages = await fetchMessages(token, params.contact_id)
    console.log(messages);
    return {
        contact, messages
    }
}