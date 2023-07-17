<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    export let outFile: string;
    let copied = false;
    
    function openFile() {
        invoke("open_file_in_default_program", {
            filePath: outFile,
        });
    }

    async function copyFile() {
        const res = await invoke("copy_file_to_clipboard", {
            filePath: outFile,
        });
        if (res === "OK") {
            copied = true;
        }
    }
</script>

<div class="flex flex-wrap justify-center gap-x-4">
    <h1 class="text-green-700 text-xl">Transcoding Successful!</h1>
    <h1
        class="text-gray-300 text-xl cursor-pointer hover:underline"
        on:click={openFile}
        on:keydown={openFile}
    >
        {outFile}
    </h1>
    <h1
        class="text-gray-300 text-xl cursor-pointer hover:underline"
        on:click={copyFile}
        on:keydown={copyFile}
    >
        {#if !copied}
            Copy file to clipboard
        {:else}
            Copied!
        {/if}
    </h1>
</div>
