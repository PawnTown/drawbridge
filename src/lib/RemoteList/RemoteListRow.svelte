<script lang="ts">
  import type { Remote } from "../../models/remote";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  const onClick = () => dispatch("click");

  export let remote: Remote;
</script>

<button on:click={onClick} class="row">
  <div class="color"></div>
  <div class="title">{ remote.label }</div>
</button>

<style scoped>
.row {
  display: block;
  width: 100%;
  position: relative;
  height: 32px;
  padding: 24px 12px;
  display: flex;
  justify-content: flex-start;
  align-items: center;
  border-radius: 12px;
  cursor: pointer;
  border: none;
  background: transparent;
  outline: none;
}

.row:hover {
  background-color: #2e2e33;
}

.row:active {
  background-color: #2a2a2f;
}

.row:hover::after {
  content: "";
  background-image: url(/chevron_right.svg);
  background-size: contain;
  background-repeat: no-repeat;
  background-position: center;
  width: 20px;
  position: absolute;
  right: 9px;
  top: 0;
  bottom: 0;
  filter: invert(1);
}

.row .color {
  width: 12px;
  height: 12px;
  background-color: #a7c4dd;
  border-radius: 4px;
}

.row .title {
  font-size: 14px;
  margin: 0 0 0 9px;
}
</style>