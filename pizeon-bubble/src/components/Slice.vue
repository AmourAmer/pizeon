<script setup lang="ts">
// use useStorage here, because I don't know how to bind value to parent
import { ModelRef } from "vue";
const props = defineProps<{
  slice: string;
}>();
const model: ModelRef<string | undefined, string> = defineModel();
const isText = function () {
  return ["heading"].indexOf(props.slice) >= 0;
};
const isSelect = function () {
  return ["server", "signature"].indexOf(props.slice) >= 0;
};
const isTextarea = function () {
  return ["body"].indexOf(props.slice) >= 0;
};
</script>

<template>
  <label :for="slice">{{ slice }}</label>
  <input v-if="isText()" type="text" :id="slice" v-model="model" />
  <textarea v-else-if="isTextarea()" :id="slice" v-model="model" />
  <select v-else-if="isSelect()" :id="slice" v-model="model" multiple>
    <option disabled value="">Please select one</option>
    <option>A</option>
    <option>B</option>
    <option>C</option>
  </select>
</template>
