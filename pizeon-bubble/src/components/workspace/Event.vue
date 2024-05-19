<script setup lang="ts">
import { Ref } from "vue";
import { useStorage } from "@vueuse/core";
import { stringMap } from "@utils/type";
import { v4 as uuidv4 } from "uuid";
import sliceTextarea from "./slice/sliceTextarea.vue";
import sliceTime from "./slice/sliceTime.vue";
import sliceHeading from "./slice/sliceHeading.vue";

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
        // TODO: clean unneeded properties
      ),
    };
    if (data.value[0].type == "heading") {
      result.heading = data.value[0].body;
    }
    // TODO: option to keep
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
    case "heading":
      return sliceHeading;
    default:
      return sliceTextarea;
  }
};

// FIXME: Yes, this is silly type. But I really don't want to make a tuple or write something like `v-if="vali == true`
const rValidateSlice: (type: string, datum: Ref<stringMap>) => boolean = (
  type: string,
  datum: stringMap,
) => {
  if (type == "heading")
    if (data.value[0] == datum) return false;
    // maybe use id?
    else {
      datum.value["type_change_type_change_warning"] =
        "Heading can only be added at the first position, click the first add button and change new item to heading";
      return true;
    }
  return false;
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
      <!-- TODO: why it says ResizeObserver loop completed with undelivered notifications. Maybe it's because display: none?! -->
      <button @click="datum.deleted = !datum.deleted">x</button>
      <!-- TODO: why cannot use v-model! -->
      <component
        :is="slice(datum.type)"
        :datum="datum"
        :servers="servers"
        :rValidator="rValidateSlice"
      />
      <i @click="delete datum.type_change_type_change_warning">{{
        datum.type_change_warning
      }}</i>
      <button @click="addItem(i + 1)">+</button>
      <!-- TODO: buttons to change type -->
      <!-- TODO: drag handle -->
    </div>
  </div>
</template>
