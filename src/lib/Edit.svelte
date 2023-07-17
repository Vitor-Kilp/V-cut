<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Video from "./Video.svelte";
    import { save } from "@tauri-apps/api/dialog";
    import { onMount } from "svelte";
    import { listen } from "@tauri-apps/api/event";
    import ProgressBar from "./ProgressBar.svelte";
    import SliderTime from "./SliderTime.svelte";
    import SusccessTranscoding from "./SusccessTranscoding.svelte";
    import parseTime from "./parseTime";
    import Options from "./Options.svelte";

    let videoDuration = 1; // All time values are in seconds
    let startTime = 0;
    let endTime = videoDuration;
    let videoBind: HTMLVideoElement | undefined;
    let progress = 0; // Transcoding progress
    let crf = "25"; // crf = Constant Rate Factor
    let paused = true;
    let preview = false;
    let encodingCpuGpu = "cpu";
    let bitrate = "10"; // In Mbits
    let outFile = "";
    let encodingType = "libx264";
    let processing = false;
    let resolution = "1080";

    export let fileUrl: string;
    export let filePath: string;
    export let showDropzone: Boolean;

    onMount(() => {
        // Listen for progress processing video
        const unlisten = listen(
            "progress",
            (message: {
                event: string;
                id: number;
                payload: string;
                windowLabel: string;
            }) => {
                let totalTime = endTime - startTime;
                const timeString = message.payload;

                // Convert the time string ( 00:00:00.000 ) into seconds
                const parsedTime = parseTime(timeString);

                // Set the progress bar %
                progress = (parsedTime / totalTime) * 100;
            }
        );

        // Remove listener on unnmount
        return () => {
            unlisten.then((unlist) => unlist());
        };
    });

    async function transcode() {
        if (filePath.length <= 0) return;

        const outPath = await save({
            filters: [
                {
                    name: "Video",
                    extensions: ["mp4"],
                },
            ],
        });

        if (!outPath) return;

        processing = true;
        outFile = "";

        if (encodingCpuGpu === "cpu") {
            const res = await invoke("transcode_video_cpu", {
                inputFile: filePath,
                outputFile: outPath,
                crf: crf,
                startTime: startTime.toString(),
                endTime: endTime.toString(),
                encoding: encodingType, //"libx265" "libx264"
                resolution: resolution,
            });
            if (res == "OK") {
                outFile = outPath;
                processing = false;
            }
        } else if (encodingCpuGpu === "gpu") {
            const res = await invoke("transcode_video_gpu", {
                inputFile: filePath,
                outputFile: outPath,
                bitrate: `${bitrate}M`,
                startTime: startTime.toString(),
                endTime: endTime.toString(),
                encoding: encodingType,
                resolution: resolution,
            });
            if (res == "OK") {
                outFile = outPath;
                processing = false;
            }
        }
    }

    // Update end time to video duration
    $: endTime = Math.floor(videoDuration * 10) / 10;

    $: {
        function checkPreview() {
            if (videoBind.currentTime >= endTime) {
                videoBind.pause();
                paused = true;
                preview = false;
                videoBind.removeEventListener("timeupdate", checkPreview);
            }
        }

        if (videoBind && preview) {
            videoBind.removeEventListener("timeupdate", checkPreview);
            videoBind.addEventListener("timeupdate", checkPreview);
        }
    }

    function handlePreview() {
        if (videoBind) {
            videoBind.currentTime = startTime;
            videoBind.play();
        }
    }

    $: if (preview) handlePreview();
   
</script>

<main class="p-6">
    <button
        class="absolute top-3 right-12 text-center text-xl text-gray-300"
        on:click={() => (showDropzone = true)}>Go back</button
    >
    <Video bind:videoBind bind:fileUrl bind:videoDuration />
    <SliderTime bind:startTime bind:endTime bind:videoDuration bind:videoBind />
    <div class="flex items-center justify-center gap-x-6 mb-4">
        <label
            class="text-gray-300 text-2xl flex justify-center items-center border-2 border-gray-300 rounded-lg p-2 transition-colors"
            style={preview && `background-color: rgb(31 41 55)`}
        >
            <input
                class="h-4 w-4 mr-4"
                type="checkbox"
                bind:checked={preview}
            />Preview
        </label>
        <button
            class="text-gray-300 text-2xl flex justify-center items-center border-2 border-gray-300 rounded-lg p-2 transition-colors"
            on:click={transcode}>Cut</button
        >
    </div>
    {#if outFile.length > 0}
        <SusccessTranscoding bind:outFile />
    {/if}
    <h1
        class="text-gray-300 text-xl border-b border-gray-300 font-semibold pb-1"
    >
        Options
    </h1>
    <Options bind:encodingCpuGpu bind:bitrate bind:crf bind:resolution bind:encodingType/>
    {#if processing}
        <ProgressBar {progress} />
    {/if}
</main>

<style>
</style>
