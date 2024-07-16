<script lang="ts">
    import {onMount} from "svelte";
    import {quintOut} from "svelte/easing";
    import {crossfade} from "svelte/transition";

    export let alertStore;

    let alert = undefined;

    onMount(() => {
        alertStore.subscribe((value) => {
            alert = value;

            setTimeout(() => {
                alert=undefined;
            }, 5000);
        });
    });

    function typeToColor(type: 'success' | 'error' | 'warning' | 'info'): string {
        switch (type) {
            case 'success':
                return '#60e394';
            case 'error':
                return '#e0786d';
            case 'warning':
                return '#eba547';
            case 'info':
                return '#3498db';
        }
    }

    function typeToColorContrast(type: 'success' | 'error' | 'warning' | 'info'): string {
        switch (type) {
            case 'success':
                return '#51d284';
            case 'error':
                return '#cd6459';
            case 'warning':
                return '#d89336';
            case 'info':
                return '#288acb';
        }
    }

    // Translate up
    const [send, receive] = crossfade({
        duration: (d) => Math.sqrt(d * 600),

        fallback(node, params) {
            const style = getComputedStyle(node);
            const transform = style.transform === 'none' ? '' : style.transform;

            return {
                duration: 600,
                easing: quintOut,
                css: (t) => `
                    transform: ${transform} translateY(${(1 - t) * 20}px);
                    opacity: ${t}
                `
            };
        }
    });
</script>

{#if alert}
    {@const alertId = Math.random()}
    <div id="alert" class="alert" style="background-color: {typeToColor(alert.type)}" in:receive={{key: alertId}} out:send={{key: alertId}}>
        <span class="message">{alert.message}</span>
        <div class="fill" style="background-color: {typeToColorContrast(alert.type)}"></div>
    </div>
{/if}

<style>
    .alert {
        position: relative;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 200px;
        height: 30px;
        padding: 1px 13px;
        transform: translateY(-50px);
        border-radius: 100px;
        color: white;
        font-weight: 500;
        font-size: 14px;
        font-family: 'DM Sans', sans-serif;
        overflow: hidden;
    }

    .message {
        position: relative;
        z-index: 2;
    }

    @keyframes popup {
        0% {
            transform: translateY(100px);
        }
        100% {
            transform: translateY(-20px);
        }
    }

    .fill {
        position: absolute;
        top: 0;
        left: 0;
        width: 0;
        z-index: 0;
        animation: gradient 5s forwards;
        height: 100%;
    }

    @keyframes gradient {
        to {
            width: 100%;
        }
    }
</style>