<script lang="ts">
    export let progress: number;
    let bar: HTMLDivElement;

    $: {
        if (bar) {
            bar.style.width = `${Math.round(progress)}%`  
        }
    }
</script>

<div class="progress-bar">
    <div class="progress" bind:this={bar} />
</div>

<style>
    .progress-bar {
        width: 100%;
        height: 20px;
        background-color: rgb(209 213 219);
        border-radius: 4px;
    }

    .progress {
        height: 100%;
        background-color: #0066ff;
        border-radius: 4px;
        transition: width 0.3s ease-in-out;
    }
</style>
