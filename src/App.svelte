<script lang="ts">

  import Dropzone from './lib/Dropzone.svelte'
  import Edit from './lib/Edit.svelte';
  
  let filePath = ''
  let fileUrl = ''
  let fileName = ''
  let showDropzone = true

</script>

<main class="p-6">
    {#if showDropzone}
    <Dropzone bind:fileName bind:fileUrl bind:filePath bind:showDropzone/>
    {:else}
    <Edit bind:fileUrl bind:filePath bind:showDropzone/>
    {/if}
</main>

<style>

</style>