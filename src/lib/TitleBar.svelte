<script lang="ts">
  import { appWindow } from '@tauri-apps/api/window'
  
  const minimize = () => appWindow.minimize();
  const close = () => appWindow.close();
</script>

<div data-tauri-drag-region class="titlebar">
  <div data-tauri-drag-region class="sides">
    <button class="titlebar-button close" on:click={close}></button>
    <button class="titlebar-button minimize" on:click={minimize}></button>
    <button class="titlebar-button"></button>
  </div>
  <div class="title">
    DrawBridge
  </div>
  <div data-tauri-drag-region class="sides"></div>
</div>

<style scoped>
  .titlebar {
    position: fixed;
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    height: 20px;
    background: transparent;
    user-select: none;
    top: 0;
    left: 0;
    right: 0;
    padding: 12px;
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
  .sides {
    display: flex;
    width: 30%;
  }
  .title {
    font-size: 14px;
    font-weight: 600;
    color: rgb(87, 87, 87);
    cursor: default;
    pointer-events: none;
    margin-top: -5px;
  }
  .titlebar-button:active {
    filter: brightness(1.2);
  }
  .titlebar:hover .close {
    background: #ff5f56;
    background-image: url('/close.svg');
    background-size: contain;
  }
  .titlebar:hover .minimize {
    background: #febc2e;
    background-image: url('/minimize.svg');
    background-size: contain;
  }
</style>