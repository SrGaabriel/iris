import {redirect} from "@sveltejs/kit";

export function load({ params }) {
    const contactId = params.contact_id;

    throw redirect(303, `/app/contacts?c=${contactId}`);
}