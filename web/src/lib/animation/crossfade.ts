import {crossfade} from "svelte/transition";
import {quintOut} from "svelte/easing";

export function createCrossfade(amplifier: number, duration: number) {
    return crossfade({
        duration: (d) => Math.sqrt(d * amplifier),

        fallback(node, params) {
            const style = getComputedStyle(node);
            const transform = style.transform === 'none' ? '' : style.transform;

            return {
                duration,
                easing: quintOut,
                css: (t) => `
				transform: ${transform} scale(${t});
				opacity: ${t}`
            };
        }
    })
}