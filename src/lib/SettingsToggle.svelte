<script>
import { createEventDispatcher } from "svelte";

const dispatch = createEventDispatcher();

const onClick = () => {
    dispatch("onPressed");
};
</script>

<button class="settings" on:click={onClick}>
    <div class="add-icon"></div>
</button>

<style>
.settings {
  border: none;
  padding: 4px;
  background-color: transparent;
  cursor: pointer;
  border-radius: 4px;
}

.settings:hover {
  background-color: rgba(58, 58, 62, 0.4);
}

.settings:active {
  background-color: rgba(58, 58, 62, 0.2);
}

.settings .add-icon {
    height: 18px;
    width: 18px;
    background-size: contain;
    background-repeat: no-repeat;
    background-image: url(/settings.svg);
    filter: invert(1) brightness(0.5);
}

.settings:hover .add-icon {
    filter: invert(1) brightness(0.7);
}
</style>