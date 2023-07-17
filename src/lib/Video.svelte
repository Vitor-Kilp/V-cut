<script lang="ts">
    export let videoBind: HTMLVideoElement | undefined;
    export let fileUrl: string;
    export let videoDuration: number;

    let showVideo = false;
    let videoSrc = fileUrl;
    let prev = "";

    $: {
        if (fileUrl.length > 0) {
            showVideo = true;
        }
    }

    $: {
        // Check if the video needs to be changed
        if (videoBind && fileUrl !== prev) {
            videoSrc = fileUrl;
            videoBind.load();
            prev = fileUrl;
        }
    }
</script>

<div class="flex justify-center items-center">
    {#if showVideo}
        <div class="relative">
            <!-- svelte-ignore a11y-media-has-caption -->
            <video
            class="rounded-md border border-gray-800"
                bind:this={videoBind}
                on:loadedmetadata={() => (videoDuration = videoBind.duration)}
                controls
            >
                <source src={fileUrl} />
            </video>
        </div>
    {/if}
</div>


<style>
</style>
