<script lang="ts">
  import FileDrop from 'svelte-tauri-filedrop'

  function open(paths: string[]) {
    // ...
  }
  
  export let value = '';

  let dragOverActive = false;

  const onDrop = (files: string[]) => {
    value = files[0];
  };
  const remove = () => {
    value = "";
  };
  const onDragOver = () => {
    dragOverActive = true;
  };
  const onDragLeave = () => {
    dragOverActive = false;
  };
</script>

<FileDrop handleFiles={onDrop} let:files>
  <div class="dropzone" class:droppable={files.length === 1}>
    {#if value === ""}
      <div class="placeholder">
        <div class="logo"></div>
        <p>Drag-n-Drop a File</p>
      </div>
    {:else}
      <div class="file">
        <button on:click={remove} class="remove"></button>
        <div class="logo"></div>
        <p>{value}</p>
      </div>
    {/if}
  </div>
</FileDrop>

<style scoped>
  .dropzone {
    background-color: rgb(9, 9, 9);
    border: dashed 2px #2d2f33;
    border-radius: 14px;
    height: 106px;
    width: 100%;
    margin: 12px 0 0 0;
  }

  .droppable {
    border: dashed 2px #0e62e2;
  }

  .placeholder {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    height: 100%;
  }

  .placeholder p{
    font-size: 12px;
    font-weight: bold;
    letter-spacing: 1.2px;
    margin: 0;
  }

  .placeholder .logo {
    background-image: url('/drop.svg');
    background-size: contain;
    background-repeat: no-repeat;
    width: 36px;
    height: 36px;
    margin: 7px 0;
    filter: invert(36%) sepia(78%) saturate(6980%) hue-rotate(213deg) brightness(95%) contrast(89%);
  }

  .file {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    height: 100%;
  }

  .file p{
    font-size: 12px;
    font-weight: bold;
    letter-spacing: 1.2px;
    margin: 0;
  }

  .file .logo {
    background-image: url('/task.svg');
    background-size: contain;
    background-repeat: no-repeat;
    width: 36px;
    height: 36px;
    margin: 7px 0;
    filter: invert(36%) sepia(78%) saturate(6980%) hue-rotate(213deg) brightness(95%) contrast(89%);
  }

  .file .remove {
    position: absolute;
    top: 12px;
    right: 12px;
    background-color: transparent;
    background-image: url('/close.svg');
    background-size: contain;
    border: none;
    width: 24px;
    height: 24px;
    filter: invert(1) brightness(0.5);
    cursor: pointer;
  }
</style>