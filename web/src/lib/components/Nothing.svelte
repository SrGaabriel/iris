<script lang="ts">
    import {onMount} from "svelte";

    type Dots = {
        x: number;
        y: number;
        radius: number;
        speedX: number;
        speedY: number;
    }

    let dots: Dots[] = [];
    let limitX: number;
    let limitY: number;

    onMount(() => {
        const nothing = document.querySelector('.nothing')!;
        limitX = 97;
        limitY = 97;
        console.log(limitX, limitY);
        animate();
    });

    function createDot() {
        const x = Math.random() * 100;
        const y = Math.random() * 100;
        const radius = Math.random() * 30;
        const speedX = Math.random() * 0.2;
        const speedY = Math.random() * 0.2;

        dots.push({x, y, radius, speedX, speedY});
    }

    function animate() {
        dots.forEach(moveDot);
        dots = dots;
        requestAnimationFrame(animate);
    }

    function moveDot(dot: Dots) {
        dot.x += dot.speedX;
        dot.y += dot.speedY;

        if (dot.x > limitX || dot.x < 0) {
            dot.speedX = -dot.speedX;
        }
        if (dot.y > limitY || dot.y < 0) {
            dot.speedY = -dot.speedY;
        }
    }

</script>

<div class="nothing">
    {#each dots as dot}
        <div class="dot" style="top: {dot.y}%; left: {dot.x}%; width: {dot.radius}px; height: {dot.radius}px;"></div>
    {/each}
</div>

<style>
    .nothing {
        position: relative;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
        height: 100%;
        color: white;
        font-size: 24px;
        margin-left: 40px;
        border-left: 1px solid #646464;
    }
    .dot {
        position: absolute;
        aspect-ratio: 1/1;
        background-color: white;
        border-radius: 100%;
        font-size: 40px;
    }
</style>