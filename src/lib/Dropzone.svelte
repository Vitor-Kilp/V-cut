<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { open } from "@tauri-apps/api/dialog";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";


  export let filePath: string;
  export let fileUrl: string;
  export let fileName: string;
  export let showDropzone: boolean;

  onMount(() => {
    const unlisten = listen("tauri://file-drop", async (event) => {

      if (Array.isArray(event.payload) && event.payload.length > 0) {
        const apiPath = convertFileSrc(event.payload.at(0).toString());

        filePath = event.payload.at(0).toString();
        fileUrl = apiPath;
        fileName = event.payload.at(0).toString().split("\\").at(-1);
        showDropzone = false;

      }
    });

    // Removes event listener on unmount
    return () => {
      unlisten.then((unlist) => unlist());
    };
  });

  const handleFileInput = async () => {
    
    const file = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          extensions: [
            "mp4",
            "mov",
            "avi",
            "mkv",
            "wmv",
            "flv",
            "m4v",
            "webm",
            "3gp",
            "mpeg",
            "mpg",
          ],
          name: "videos",
        },
      ],
    });

    if (file) {
      const path = Array.isArray(file) ? file[0] : file;
      const apiPath = convertFileSrc(path);

      filePath = path;
      fileUrl = apiPath;
      fileName = path;
      showDropzone = false;
      
    }
  };
</script>

<div
  class="flex flex-col gap-y-5 justify-center items-center h-[93vh] w-full border-dashed border-2 border-gray-400"
>
  <h1 class="text-3xl text-gray-300">Drop your video here:</h1>
  <button
    class="text-3xl text-gray-300 p-2 border rounded-lg hover:bg-gray-900 transition-colors delay-75 ease-in-out"
    on:click={handleFileInput}>Or choose the file</button
  >
  <h1 class="text-3xl text-gray-300">{fileName}</h1>
</div>
