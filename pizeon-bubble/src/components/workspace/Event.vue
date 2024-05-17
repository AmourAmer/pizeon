<script setup lang="ts">
import { Ref } from "vue";
import { useStorage } from "@vueuse/core";
import { stringMap } from "@utils/type";
import { v4 as uuidv4 } from "uuid";
import sliceTextarea from "./slice/sliceTextarea.vue";

// TODO: another storage name
const data: Ref<stringMap[]> = useStorage("event", []);
defineProps<{
  servers: string[];
}>();

defineExpose({
  finalize() {
    const result: { heading?: any; raw: stringMap[] } = {
      raw: data.value.filter(
        (item) => !item.deleted && delete item.deleted && delete item.id,
      ),
    };
    if (data.value[0].type == "heading") {
      result.heading = data.value[0].body;
    }
    data.value = [];
    return result;
  },
});

const addItem = (idx: number) => {
  data.value.splice(idx, 0, {
    id: uuidv4(),
    type: "text",
  });
};

const slice = (type: string) => {
  switch (type) {
    default:
      return sliceTextarea;
  }
};
</script>

<template>
  <div>
    <div>{{ servers }}, {{ data }}</div>
    <button @click="addItem(0)">+</button>
    <div
      v-for="(datum, i) in data"
      :key="datum.id"
      style="
        border: 1px solid black;
        margin: 3px;
        display: flex;
        justify-content: center;
      "
    >
      <!-- TODO: why it says ResizeObserver loop completed with undelivered notifications. -->
      <button @click="datum.deleted = !datum.deleted">x</button>
      <!-- TODO: why cannot use v-model! -->
      <!-- TODO: Unhandled Promise Rejection: Maximum recursive updates exceeded in component <Event>. This means you have a reactive effect that is mutating its own dependencies and thus recursively triggering itself. Possible sources include co... -->
      <component :is="slice(datum.type)" :datum="datum" :servers="servers" />
      <button @click="addItem(i + 1)">+</button>
    </div>
  </div>
</template>
