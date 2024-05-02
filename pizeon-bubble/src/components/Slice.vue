<script setup lang="ts">
// use useStorage here, because I don't know how to bind value to parent
import { ModelRef } from "vue";
const props = defineProps<{
  slice: string;
  server: [string];
}>();
const model: ModelRef<string | undefined, string> = defineModel();
const isSelect = function () {
  return ["server", "signature"].indexOf(props.slice) >= 0;
};
const isTextarea = function () {
  return ["body"].indexOf(props.slice) >= 0;
};

const initMsg = () => {
  switch (props.slice) {
    case "body":
      return props.server.length > 0
        ? "What are you going to publish on " +
            props.server.slice(0, -1).join(", ") +
            (props.server.length > 1 ? ", and " : "") +
            props.server.slice(-1) +
            "?"
        : "Please select a server on which you are going to publish your notice";
  }
};
</script>

<template>
  {{ model }}
  <label :for="slice">{{ slice }}</label>
  <textarea
    v-if="isTextarea()"
    :id="slice"
    v-model="model"
    :placeholder="initMsg()"
  />
  <select v-else-if="isSelect()" :id="slice" v-model="model" multiple>
    <!-- TODO: shortcut multi-select -->
    <option disabled value="">Please select one</option>
    <option>A</option>
    <option>B</option>
    <option>C</option>
  </select>
  <input v-else type="text" :id="slice" v-model="model" />
</template>
