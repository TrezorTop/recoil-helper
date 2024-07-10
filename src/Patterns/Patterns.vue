<script setup lang="ts">
import { BackendService } from "../services/backend.service.ts";
import { computed, ref } from "vue";

const config = ref(BackendService.getConfig());
const search = ref<string>("");

const searchedConfigs = computed(() => {
  return Object.keys(config.value.patterns).filter((key) =>
    key.toLowerCase().includes(search.value.toLowerCase()),
  );
});

const renamePattern = (oldPatternName: string, newPatternName: string) => {
  if (oldPatternName !== newPatternName) {
    config.value.patterns[newPatternName] =
      config.value.patterns[oldPatternName];
    delete config.value.patterns[oldPatternName];
  }

  saveConfig();
};
const addStep = (patternName: string) => {
  config.value.patterns[patternName].push({
    x: 0,
    y: 0,
    delay: 0,
  });
};
const deleteStep = (patternName: string, index: number) => {
  config.value.patterns[patternName].splice(index, 1);
};
const addPattern = () => {
  const name = "New Pattern";
  let index = 0;

  while (config.value.patterns[`${name} ${index}`]) {
    index++;
  }

  config.value.patterns[`${name} ${index}`] = [];
};
const deletePattern = (patternName: string) => {
  delete config.value.patterns[patternName];
};

const saveConfig = () => {
  BackendService.saveConfig(config.value);
};
</script>

<template>
  <div class="patterns">
    <h2>Patterns</h2>

    <input v-model="search" placeholder="Search Patterns" />

    <div
      class="pattern"
      v-for="(pattern, index) in searchedConfigs"
      :key="index"
    >
      <div class="pattern-controls">
        <div class="name">
          <form
            @submit.prevent="
              renamePattern(
                pattern,
                (($event.target as HTMLFormElement)[0] as HTMLInputElement)
                  .value,
              )
            "
          >
            <input :value="pattern" />

            <button type="submit">Save</button>
          </form>
        </div>

        <button @click="deletePattern(pattern)">Delete Pattern</button>
      </div>

      <div
        class="pattern-part"
        v-for="(patternPart, index) in config.patterns[pattern]"
        :key="index"
      >
        <div v-for="(_, key) in patternPart" :key="key">
          {{ key }}: <input type="number" v-model="patternPart[key]" />
        </div>

        <button class="delete" @click="deleteStep(pattern, index)">
          Delete Step
        </button>
      </div>

      <button @click="addStep(pattern)">Add Step</button>
    </div>

    <button @click="addPattern">Add Pattern</button>
    <button @click="saveConfig">Save Config</button>
  </div>
</template>

<style scoped>
.patterns {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.pattern {
  display: flex;
  flex-direction: column;
  gap: 16px;

  margin-left: 16px;
}

.pattern-controls {
  display: flex;
  justify-content: space-between;
}

.pattern-controls .name form {
  display: flex;
  gap: 12px;
}

.pattern-part {
  position: relative;
  padding: 12px;

  border: 1px solid black;
}

.delete {
  position: absolute;

  top: 12px;
  right: 12px;
}
</style>
