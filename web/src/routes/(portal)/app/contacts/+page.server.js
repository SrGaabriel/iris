export function load({ url }) {
    const contactId = url.searchParams.get('c');
    return {
        requestedContactId: contactId
    };
}