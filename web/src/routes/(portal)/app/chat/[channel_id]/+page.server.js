import {redirect} from "@sveltejs/kit";

export function load({ params }) {
    const channelId = params.channel_id;
    throw redirect(303, `/app/chat?c=${channelId}`);
}