<script lang="ts">
    export let videoDuration: number;
    export let startTime: number;
    export let endTime: number;
    export let videoBind: HTMLVideoElement;

    const handleStartChange = (event: Event) => {
        const htmlTarget = event.target as HTMLInputElement;
        const value = parseFloat(htmlTarget.value);
        if (value >= 0 && value < endTime) {
            startTime = value;
            if (videoBind) videoBind.currentTime = value;
        } else {
            startTime = value;
            endTime = startTime;
        }
    };

    const handleEndChange = (event: Event) => {
        const htmlTarget = event.target as HTMLInputElement;
        const value = parseFloat(htmlTarget.value);
        if (value > startTime && value <= videoDuration) {
            endTime = value;
        } else {
            endTime = value;
            startTime = endTime;
        }
    };
</script>

<div class="flex relative w-full items-center justify-center">
    <svg
        class="absolute top-4"
        version="1.1"
        id="line_2"
        xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink"
        x="0px"
        y="0px"
        width="98%"
        height="3px"
        xml:space="preserve"
    >
        <path fill="#fff" stroke-width="4" stroke="#fff" d="M0 0 l5000 0" />
    </svg>
    <input
        class="absolute top-4"
        id="input1"
        type="range"
        min={0}
        max={videoDuration}
        value={startTime}
        on:input={handleStartChange}
        step={0.1}
    />
    <input
        class="absolute top-4"
        id="input2"
        type="range"
        min={0}
        max={videoDuration}
        value={endTime}
        on:input={handleEndChange}
        step={0.1}
    />
    <div class="flex justify-between w-full mt-10">
        <p class="text-gray-300 text-lg">Start Time: {startTime}s</p>
        <p class="text-gray-300 text-lg">End Time: {endTime}s</p>
    </div>
</div>

<style>
    #input1::-webkit-slider-thumb {
        -webkit-appearance: none;
        pointer-events: all;
        width: 24px;
        height: 24px;
        background-color: #0066ff;
        border-radius: 50%;
        cursor: pointer;
    }
    #input2::-webkit-slider-thumb {
        -webkit-appearance: none;
        pointer-events: all;
        width: 24px;
        height: 24px;
        background-color: #ff8800;
        border-radius: 50%;
        cursor: pointer;
    }

    #input1[type="range"],
    #input2[type="range"] {
        -webkit-appearance: none;
        appearance: none;
        height: 0px;
        width: 100%;
        position: absolute;
        pointer-events: none;
    }
</style>
