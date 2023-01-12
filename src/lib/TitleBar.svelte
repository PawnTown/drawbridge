<script lang="ts">
  import { appWindow } from '@tauri-apps/api/window'
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher, onMount } from 'svelte';
  
  const minimize = () => appWindow.minimize();
  const close = () => appWindow.close();

  let os: string = "";

  onMount(() => {
    (async () => {
      os = await invoke("get_os");
    })();
  });
</script>

<div data-tauri-drag-region class={`titlebar${os === "win" ? " win" : ""}`}>
  <div data-tauri-drag-region class="sides-unix">
    {#if os !== "win"}
    <button class="titlebar-button close" on:click={close}></button>
    <button class="titlebar-button minimize" on:click={minimize}></button>
    <button class="titlebar-button"></button>
    {/if}
  </div>
  <div class="title">
    DrawBridge
  </div>
  <div data-tauri-drag-region class="sides-win">
    {#if os === "win"}
    <button class="titlebar-button-win minimize-win" on:click={minimize}></button>
    <button class="titlebar-button-win close-win" on:click={close}></button>
    {/if}
  </div>
</div>

<style scoped>
  .titlebar {
    position: fixed;
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    height: 28px;
    background: transparent;
    user-select: none;
    top: 0;
    left: 0;
    right: 0;
  }
  .titlebar-button {
    display: flex;
    flex-grow: 0;
    flex-shrink: 0;
    justify-content: center;
    align-items: center;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    border: none;
    padding: 0;
    margin-right: 9px;
    background: #2a2c30;
    cursor: default;
  }
  .sides-unix {
    display: flex;
    width: 30%;
    padding: 12px;
  }
  .title {
    font-size: 14px;
    font-weight: 600;
    color: rgb(87, 87, 87);
    cursor: default;
    pointer-events: none;
    margin-top: 7px;
  }
  .titlebar-button:active {
    filter: brightness(1.2);
  }
  .titlebar:hover .close {
    background: #ff5f56;
    background-image: url('/close.svg');
    background-size: contain;
    background-repeat: no-repeat;
  }
  .titlebar:hover .minimize {
    background: #febc2e;
    background-image: url('/minimize.svg');
    background-size: contain;
    background-repeat: no-repeat;
  }

  .sides-win {
    position: relative;
    display: flex;
    width: 30%;
    height: 100%;
    justify-content: flex-end;
  }

  .titlebar-button-win {
    position: relative;
    width: 42px;
    height: 100%;
    margin: 0;
    border: none;
    padding: 0;
    background-color: transparent;
    transition: ease-in 0.1s;
  }

  .titlebar-button-win.close-win::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;
    background-image: url('/close.svg');
    background-size: 20px;
    background-repeat: no-repeat;
    background-position: center center;
    filter:  invert(1);
  }

  .titlebar-button-win.minimize-win::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;
    background-image: url('/minimize.svg');
    background-size: 20px;
    background-repeat: no-repeat;
    background-position: center center;
    filter:  invert(1);
  }

  .titlebar-button-win.minimize-win:hover {
    background-color: rgba(190, 190, 190, 0.1);
  }

  .titlebar-button-win.close-win:hover {
    background-color: #e81123;
  }
</style>