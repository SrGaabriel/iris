import {redirect} from "@sveltejs/kit";
import {API} from "../../interaction/server.ts";
import type {Self} from "$lib/user.ts";

export async function load({ cookies, url }) {
    if (!cookies.get('token')) {
        throw redirect(303, `/login?redirect=${url.pathname}`);
    }
    const token = cookies.get('token');
    if (!token) {
        throw redirect(303, `/login?redirect=${url.pathname}`);
    }
    const response = await fetch(`${API}/api/users/@me`, {
        headers: {
            'Authorization': `Bearer ${token}`
        }
    });
    if (response.ok) {
        const user: Self = await response.json();
        return {user, token};
    } else {
        throw redirect(303, `/login?redirect=${url.pathname}`);
    }
}