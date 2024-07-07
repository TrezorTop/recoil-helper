<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api";

const hotkeys = ref<string[]>([]);

const onAdd = () => {
  hotkeys.value.push("");
};

const onKeydown = (event: KeyboardEvent, index: number) => {
  console.log(event);
  hotkeys.value[index] = event.key;
};
const onMousedown = (event: MouseEvent, index: number) => {
  console.log(event);
  hotkeys.value[index] = `MB_${event.button}`;
};

const onSave = () => {
  invoke("set_hotkeys_for_action", {
    action: "test_action",
    hotkeys: hotkeys.value,
  });
};
</script>

<template>
  <div>choose hotkeys:</div>
  <div class="hotkeys">
    <input
      class="hotkey"
      v-for="(hotkey, index) in hotkeys"
      :value="hotkeys[index]"
      :key="hotkey"
      @keydown="onKeydown($event, index)"
      @mousedown="onMousedown($event, index)"
    />
    <button @click="onAdd">+</button>
  </div>
  {{ hotkeys }}
  <button @click="onSave">save</button>
</template>

<style scoped>
.hotkeys {
  display: flex;
  gap: 4px;
}

.hotkey {
  height: 21px;
  border: 1px solid black;
}
</style>
`
