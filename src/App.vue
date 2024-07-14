<script setup lang="ts">
import { BackendService } from "./services/backend.service.ts";

const config = BackendService.getConfig();

const onPatternSelect = (name: string) => {
  BackendService.setActivePattern(name);
};
</script>

<template>
  <div class="app">
    <button
      v-for="(_, key) in config.patterns"
      :key="key"
      @click="onPatternSelect(key as string)"
    >
      {{ key }}
    </button>
  </div>
</template>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  gap: 12px;

  padding: 32px;
}
</style>
