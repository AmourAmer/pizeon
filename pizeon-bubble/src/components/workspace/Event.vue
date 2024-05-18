<script setup lang="ts">
import { Ref } from "vue";
import { useStorage } from "@vueuse/core";
import { stringMap } from "@utils/type";
import { v4 as uuidv4 } from "uuid";
import sliceTextarea from "./slice/sliceTextarea.vue";
import sliceTime from "./slice/sliceTime.vue";

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
    case "text":
      return sliceTextarea;
    case "time":
      return sliceTime;
    default:
      return sliceTextarea;
  }
};

// FIXME: Yes, this is silly type. But I really don't want to make a tuple or write something like `v-if="vali == true`
const rValidateSlice: (type: string, datum: stringMap) => false | string = (type: string, datum: stringMap) => {
  return false;
};
</script>

<template>
  <div>
    <div>{{ servers }}, {{ data }}</div>
    <button @click="addItem(0)">+</button>
    <div v-for="(datum, i) in data" :key="datum.id" style="
        border: 1px solid black;
        margin: 3px;
        display: flex;
        justify-content: center;
      ">
      <!-- TODO: why it says ResizeObserver loop completed with undelivered notifications. -->
      <button @click="datum.deleted = !datum.deleted">x</button>
      <!-- TODO: why cannot use v-model! -->
      <component :is="slice(datum.type)" :datum="datum" :servers="servers" :rValidator="rValidateSlice" />
      <button @click="addItem(i + 1)">+</button>
      <!-- TODO: buttons to change type -->
    </div>
  </div>
</template>
