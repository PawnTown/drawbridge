<script lang="ts">
  import { editor } from 'monaco-editor';
  import { fly, fade } from 'svelte/transition';
  import { createEventDispatcher, onMount } from 'svelte';
  import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
	import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';
	import cssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker';
	import htmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker';
	import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker';
  import { saveRemote, type Remote } from '../models/remote';
  import { GetStore } from '../services/storage';

  const defaultMiddleware = `function message_in(line)
  return line;
end;

function message_out(line)
  --[[Example: Filter out Hash overwrites--]]
  --[[
  filterPrefix = "setoption name Hash"
  if string.sub(line, 1, string.len(filterPrefix))==filterPrefix then
    return ""
  end
  --]]
  return line;
end;
`;

	//@ts-ignore
	self.MonacoEnvironment = {
		getWorker(_, label) {
			if (label === 'json') {
				return new jsonWorker();
			}
			if (label === 'css' || label === 'scss' || label === 'less') {
				return new cssWorker();
			}
			if (label === 'html' || label === 'handlebars' || label === 'razor') {
				return new htmlWorker();
			}
			if (label === 'typescript' || label === 'javascript') {
				return new tsWorker();
			}
			return new editorWorker();
		}
	};

  export let remote: Remote;

  let editorEl: HTMLElement;
  let editorInstace: editor.IStandaloneCodeEditor;
  const dispatch = createEventDispatcher();
  const store = GetStore();

  onMount(() => {
    editorInstace = editor.create(editorEl, {
      value: remote.middlewareLua || defaultMiddleware,
      language: 'lua',
      theme: "vs-dark",
      lineNumbers: 'off',
      glyphMargin: false,
      folding: false,
      lineDecorationsWidth: 0,
      lineNumbersMinChars: 0,
      minimap: {
        enabled: false
      },
    });
  });

  const onSave = async () => {
    const remotes: Remote[] = await store.get('remotes') ?? [];
    remote.middlewareLua = editorInstace.getValue();
    const newRemotes = saveRemote(remotes, remote);
    await store.set("remotes", newRemotes);
    dispatch("success", { remote: newRemotes[newRemotes.length - 1] });
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
  
  <div class="editor-wrap">
    <div class="editor" bind:this={editorEl} style="width:100%;height:100%;"></div>
  </div>

  <div>
    <button class="submit" on:click={onSave}>Save</button>
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
    height: 420px;
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

  .editor-wrap {
    position: relative;
    margin-top: 32px;
    height: 320px;
    padding: 7px;
    box-sizing: border-box;
    background: #1e1e1e;
    border-radius: 7px;
    text-align: left;
  }

  .editor {
    height: 100%;
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