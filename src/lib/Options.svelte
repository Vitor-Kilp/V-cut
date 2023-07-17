<script lang="ts">
    export let encodingCpuGpu: string;
    export let crf: string;
    export let bitrate: string;
    export let encodingType: string;
    export let resolution: string;
</script>

<div class="flex my-5 flex-wrap items-center gap-x-5 gap-y-4">
    <small class="text-gray-300 text-xl">Transcode Using:</small>
    <select
        class="bg-black border border-gray-300 text-gray-300 text-lg rounded-lg p-1.5 font-semibold"
        bind:value={encodingCpuGpu}
        on:change={(e) => (encodingCpuGpu = e.currentTarget.value)}
    >
        <option class="text-lg" selected value="cpu">CPU</option>
        <option class="text-lg" value="gpu">GPU</option>
    </select>
    {#if encodingCpuGpu === "cpu"}
        <div class="flex items-center gap-x-3">
            <small class="text-gray-300 text-xl">Quality:</small>
            <input
                type="range"
                min={15}
                max={45}
                value={crf}
                on:input={(e) => (crf = crf = e.currentTarget.value)}
                step={1}
            />
            <small class="text-gray-300 text-lg"
                >{crf} crf
                <small class="text-gray-400 text-center"
                    >(smaller is better)</small
                ></small
            >
        </div>
    {:else if encodingCpuGpu === "gpu"}
        <div class="flex items-center gap-x-3">
            <small class="text-gray-300 text-lg">Quality:</small>
            <input
                type="range"
                min={1}
                max={40}
                value={bitrate}
                on:input={(e) => (bitrate = e.currentTarget.value)}
                step={1}
            />
            <small class="text-gray-300 text-lg"
                >{bitrate} M/bits
                <small class="text-gray-400 text-center"
                    >(bigger is better)</small
                ></small
            >
        </div>
    {/if}
    <div class="flex items-center gap-x-3">
        <small class="text-gray-300 text-xl">Transcoding Algorithm:</small>
        <select
            class="bg-black border border-gray-300 text-gray-300 text-lg rounded-lg p-1.5 font-semibold"
            bind:value={encodingType}
            on:change={(e) => (encodingType = e.currentTarget.value)}
        >
            <option selected value="libx264">H.264</option>
            <option value="libx265">H.265</option>
        </select>
    </div>
    <div class="flex items-center gap-x-3">
        <small class="text-gray-300 text-xl">Resolution:</small>
        <select
            class="bg-black border border-gray-300 text-gray-300 text-lg rounded-lg p-1.5 font-semibold"
            bind:value={resolution}
            on:change={(e) => (resolution = e.currentTarget.value)}
        >
            <option class="text-lg" selected value="2160">2160p</option>
            <option class="text-lg" selected value="1080">1080p</option>
            <option class="text-lg" value="720">720p</option>
            <option class="text-lg" value="480">480p</option>
            <option class="text-lg" value="360">360p</option>
        </select>
    </div>
</div>
