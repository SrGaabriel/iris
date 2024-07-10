import {redirect} from "@sveltejs/kit";
import {DOMAIN} from "../../interaction/server.ts";
import type {Self} from "$lib/user.ts";

export async function load({ cookies, url }) {
    if (!cookies.get('token')) {
        console.log('No token found 0');
        throw redirect(303, `/login?redirect=${url.pathname}`);
    }
    const token = cookies.get('token');
    if (!token) {
        console.error('No token found');
        throw redirect(303, `/login?redirect=${url.pathname}`);
    }
    console.log("Wtf2");
    const response = await fetch(`http://${DOMAIN}/api/users/@me`, {
        headers: {
            'Authorization': `Bearer ${token}`
        }
    });
    console.log("Wtf");
    console.log(response);
    if (response.ok) {
        const user: Self = await response.json();
        return {user};
    } else {
        console.error('No token found 1');
        throw redirect(303, `/login?redirect=${url.pathname}`);
    }
}