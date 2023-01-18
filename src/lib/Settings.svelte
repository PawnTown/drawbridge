<script lang="ts">
  import { fly, fade } from 'svelte/transition';
  import { open, save } from '@tauri-apps/api/dialog';
  import { createEventDispatcher, onMount } from 'svelte';
  import type { SettingsModel } from 'src/models/settings';
  import { invoke } from '@tauri-apps/api/tauri';
  import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
  import { relaunch } from '@tauri-apps/api/process'
  import { OpenUrl } from '../services/open';
  import CheckBox from './CheckBox.svelte';

  export let settings: SettingsModel;

  const dispatch = createEventDispatcher();

  let os: string = "";
  let updateAvailable = false;
  let loading = false;

  onMount(() => {
    (async () => {
      os = await invoke("get_os");
    })();
  });

  const success = () => {
      dispatch("success", settings);
  };

  const onClose = () => {
      dispatch("close");
  };

  const browse = async () => {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'Executable',
          extensions: ['exe'],
        },
      ],
    });
    if (!Array.isArray(selected) && selected !== null) {
      settings.csCompilerPath = selected;
    }
  }

  const browseSave = async () => {
    const selected = await save({
      filters: [
        {
          name: 'Textfile',
          extensions: ['txt'],
        },
      ],
    });
    if (selected !== null) {
      settings.logFile = selected;
    }
  }

  const checkForUpdates = async () => {
    try {
      loading = true;
      const { shouldUpdate, manifest } = await checkUpdate()
      updateAvailable = shouldUpdate
      loading = false;
    } catch (error) {
      console.log(error)
    }
  }

  const doUpdate = async () => {
    await OpenUrl('https://pawntown.github.io/drawbridge_web/');
    // TODO: real updater
    /*try {
      loading = true;
      // display dialog
      await installUpdate()
      // install complete, restart the app
      await relaunch()
      loading = false;
    } catch (error) {
      console.log(error)
    }*/
  }
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
    <div class="update-card">
      {#if updateAvailable}
        <h2>🔥New Update Available!</h2>
        <p>Your DrawBridge Version is outdated. Update now to get the newest features and fixes.</p>
        <div class="button-wrap">
          <button on:click={doUpdate} class="install-update">{!loading ? "♻️ Install Update" : "..."}</button>
        </div>
      {/if}

      {#if !updateAvailable}
        <h2>🤘🏽DrawBridge is up to date!</h2>
        <p>DrawBridge is currently up to date. Check back later for updates.</p>
        <div class="button-wrap">
          <button on:click={checkForUpdates} class="check-for-updates">{!loading ? "Check for updates" : "..."}</button>
        </div>
      {/if}
    </div>

    <div class="input-wrap file" class:disabled={!settings.enableLogs}>
      <div class="split">
        <CheckBox title="Enable Logs" bind:value={settings.enableLogs} />
        <div class="question">
          <div class="logo"></div>
          <div class="content">
            <h3>Placeholder</h3>
            <ul>
              <li><b>&#123;rnd&#125;</b> Random Generated String</li>
              <li><b>&#123;ts&#125;</b> Timestamp (Nanosecond)</li>
            </ul>
          </div>
        </div>
      </div>
      <input disabled={!settings.enableLogs} type="text" autocorrect="off" autocapitalize="none" placeholder={os === "win" ? "C:\\Users\\Joe\\Desktop\\log.txt" : "/Users/joe/Desktop/log.txt"} bind:value={settings.logFile} />
      <button disabled={!settings.enableLogs} on:click={browseSave} class="browse">...</button>
    </div>

    {#if os === "win"}
    <div class="input-wrap label file">
      <span>Custom C#-Compiler Path</span>
      <input type="text" autocorrect="off" autocapitalize="none" placeholder="C:\Windows\Microsoft.NET\Framework\v4.0.30319\csc.exe" bind:value={settings.csCompilerPath} />
      <button on:click={browse} class="browse">...</button>
    </div>
    {/if}
  </div>

  <div>
    <button class="submit" on:click={success}>Save</button>
  </div>
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
    height: 480px;
    display: flex;
    justify-content: space-between;
    flex-direction: column;
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
    margin: 14px 0;
  }

  .input-wrap.label {
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

  .input-wrap.disabled input {
    opacity: 0.4;
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

  .input-wrap.file input {
    padding-right: 60px;
  }

  .input-wrap.file button {
    position: absolute;
    right: 3px;
    bottom: 3px;
    width: 54px;
    height: 34px;
    border-radius: 9px;
    border: none;
    font-size: 18px;
    background-color: #0175ff;
    cursor: pointer;
  }

  .input-wrap.file button:hover {
    background-color: #1d82ff;
  }

  .input-wrap.file button:active {
    background-color: #1065cb;
  }

  .input-wrap.disabled.file button {
    background-color: rgba(98, 102, 107, 1);
    opacity: 0.3;
    cursor: default;
  }

  .input-wrap.disabled.file button:hover {
    background-color: rgba(98, 102, 107, 1);
  }

  .input-wrap.disabled.file button:active {
    background-color: rgba(98, 102, 107, 1);
  }

  .update-card {
    position: relative;
    width: 100%;
    background-color: #272b30;
    border-radius: 12px;
    padding: 16px;
    box-sizing: border-box;
    text-align: left;
    flex-grow: 0;
    margin: 14px 0 0 0;
  }

  .update-card h2 {
    font-size: 14px;
    font-weight: bold;
    margin: 0;
    padding: 0;
  }

  .update-card p {
    font-size: 14px;
    color: rgb(194, 194, 194);
    margin: 0;
    padding: 0;
    margin: 4px 0 0 0;
  }

  button.check-for-updates {
    height: 33px;
    width: 200px;
    margin-top: 12px;
    border-radius: 9px;
    border: none;
    font-size: 12px;
    background-color: #0175ff;
    cursor: pointer;
  }

  button.check-for-updates:hover {
    background-color: #1d82ff;
  }

  button.check-for-updates:active {
    background-color: #1065cb;
  }

  button.install-update {
    height: 33px;
    width: 200px;
    margin-top: 12px;
    border-radius: 9px;
    border: none;
    font-size: 12px;
    font-weight: bold;
    background-color: rgb(34, 109, 90);
    cursor: pointer;
  }

  button.install-update:hover {
    background-color: rgb(48, 134, 113);
  }

  button.install-update:active {
    background-color: rgb(27, 94, 77);
  }

  .button-wrap {
    margin-top: 12px;
    display: flex;
    justify-content: center;
  }

  .split {
    display: flex;
    justify-content: space-between;
  }

  .question {
    width: 24px;
    height: 24px;
    padding: 2px;
    border-radius: 4px;
    box-sizing: border-box;
  }

  .question:hover {
    background: #272b30;
  }

  .question .content {
    position: absolute;
    top: -42px;
    right: 32px;
    background: rgb(22, 20, 20);
    text-align: left;
    font-size: 12px;
    z-index: 2;
    border-radius: 12px;
    border: 4px solid rgba(135, 189, 255, 0.3);
    opacity: 0;
    pointer-events: none;
    transform: translateX(14px);
    transition: cubic-bezier(0.215, 0.610, 0.355, 1) 0.2s;
  }

  .question .content h3 {
    padding: 12px;
    background: #0175ff;
    border-radius: 9px 9px 0 0;
    margin: 0;
  }

  .question .content ul {
    list-style: none;
    margin: 0;
    padding: 12px 24px;
  }

  .question .content ul li::before {
    content: "-";
    color: #0175ff;
    font-weight: bold;
    font-size: 16px;
    padding-right: 7px;
  }

  .question:hover .content {
    opacity: 1;
    transform: translateX(0);
  }

  .question .logo {
    width: 100%;
    height: 100%;
    background-image: url(/question.svg);
    background-size: contain;
    background-position: center center;
    filter: invert(1) brightness(0.6);
  }
</style>