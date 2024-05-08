<script setup lang="ts">
import { ModelRef, onMounted } from "vue";

const props = defineProps<{
  slice: string;
  server: string[];
}>();
const model: ModelRef<string | undefined, string> = defineModel();
const selectFor = function (slice: string) {
  // TODO: use real back-end fn calls, should store data in kinda table
  if (["signature"].indexOf(slice) >= 0) return ["A", "B", "C", "D", "self"];
  if (["server"].indexOf(slice) >= 0) return ["A", "B", "C", "D", "self"];
  return [];
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

onMounted(() => {
  // TODO: auto or enter, remember which to focus
  if (props.slice == "heading") {
    document.getElementById(props.slice)?.focus();
  }
});
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
  <select
    v-else-if="selectFor(slice).length > 0"
    :id="slice"
    v-model="model"
    multiple
  >
    <!-- TODO: shortcut multi-select -->
    <!-- TODO: default choice -->
    <option disabled value="">Please select one or more, I don't know</option>
    <option v-for="(option, i) in selectFor(slice)" :key="i">
      {{ option }}
    </option>
  </select>
  <input v-else type="text" :id="slice" v-model="model" />
</template>
