<script lang="ts">
  import { save } from '@tauri-apps/api/dialog';
  import { invoke } from "@tauri-apps/api/tauri";
  import { fly, fade } from 'svelte/transition';
  import { createEventDispatcher } from 'svelte';
  import type { Remote } from '../models/remote';
  import { GetStore } from '../services/storage';
    import type { SettingsModel } from 'src/models/settings';

  const store = GetStore();

  export let settings: SettingsModel;

  let deleteUIVisible = false;

  const dispatch = createEventDispatcher();

  const success = () => {
      dispatch("success", settings);
  };

  const onClose = () => {
      dispatch("close");
  };
</script>
  
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
  class="backdrop"
  out:fade="{{ duration: 250 }}"
  in:fade="{{ duration: 250 }}"
  on:click={onClose}
></div>

<div
  class="dialog details"
  out:fly="{{ y: 250, duration: 250, delay: 0 }}"
  in:fly="{{ y: 250, duration: 250 }}"
>
  <button class="cancel" on:click={onClose}></button>
  
  <div class="inner-wrap">
    <div class="input-wrap prefix-icon label">
      <span>C#-Compiler Path</span>
      <input type="text" autocorrect="off" autocapitalize="none" placeholder="My Awesome Cluster" bind:value={settings.csCompilerPath} />
    </div>
  </div>

  <button class="submit" on:click={success}>Save</button>
</div>

<style>
  .backdrop {
    position: absolute;
    background-color: rgba(0, 0, 0, 0.84);
    top: 34px;
    bottom: 0;
    left: 0;
    right: 0;
  }

  .dialog {
    border-radius: 12px 12px 0 0;
    position: absolute;
    background-color: #101113;
    bottom: 0;
    left: 0;
    right: 0;
    padding: 12px;
  }

  .dialog.details {
    height: 390px;
  }

  .cancel {
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

  .cancel:hover {
    filter: invert(1) brightness(1);
  }

  .inner-wrap {
    position: relative;
    margin: 20px;
    box-sizing: border-box;
    width: calc(100% - 40px);
  }

  input {
    width: 100%;
    box-sizing: border-box;
    background-color: #272b30;
    border: none;
    padding: 12px;
    border-radius: 9px;
    font-size: 14px;
    outline: none;
  }

  input::placeholder {
    color: #4c5257;
  }

  .input-wrap {
    position: relative;
    width: 100%;
    margin: 7px 0;
    padding-top: 24px;
  }

  .input-wrap span {
    position: absolute;
    display: block;
    left: 9px;
    top: 0;
    font-size: 14px;
    color: #62666b;
  }

  button.submit {
    background-color: #0175ff;
    border: none;
    padding: 14px 32px;
    border-radius: 12px;
    margin: 24px 0 0 0;
    font-weight: bold;
    font-size: 12px;
    cursor: pointer;
  }

  button.submit:hover {
    background-color: #1d82ff;
  }

  button.submit:active {
    background-color: #1065cb;
  }
</style>