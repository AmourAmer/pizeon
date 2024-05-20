<script setup lang="ts">
import { Ref } from "vue";
import { stringMap } from "@utils/type";
import { useData } from "src/utils/draft";
import { v4 as uuidv4 } from "uuid";
import sliceTitle from "./slice/sliceTitle.vue";
import sliceHost from "./slice/sliceHost.vue";
import sliceTime from "./slice/sliceTime.vue";
import slicePlace from "./slice/slicePlace.vue";
import sliceTextarea from "./slice/sliceTextarea.vue";

// TODO: draggable, not so urgent
// TODO: another storage name
const { data, nonDeletedIter, uniqueDataType } = useData("event");
defineProps<{
  servers: string[];
}>();

defineExpose({
  finalize() {
    let nonDeletedData = [...nonDeletedIter(data.value)];
    const result: { title?: any; raw: stringMap[] } = {
      raw: nonDeletedData.filter(
        (item) => !item.deleted && delete item.deleted && delete item.id,
        // TODO: clean unneeded properties, maybe call fn of child components
      ),
    };
    if (nonDeletedData[0].type == "title") {
      result.title = nonDeletedData[0].body;
    }
    // TODO: option to keep
    data.value = [];
    return result;
  },
  // TODO: maybe need a more appropriate name, and make finalize use this
  preview() {
    let nonDeletedData = [...nonDeletedIter(data.value)];
    const result: { title?: any; raw: stringMap[] } = {
      raw: nonDeletedData.filter((item) => !item.deleted),
    };
    if (nonDeletedData[0].type == "title") {
      result.title = nonDeletedData[0].body;
    }
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
    case "title":
      return sliceTitle;
    case "host":
      return sliceHost;
    case "time":
      return sliceTime;
    case "place":
      return slicePlace;
    case "text":
      return sliceTextarea;
    default:
      return sliceTextarea;
  }
};

// FIXME: deprecate all other rVali's
// should let existing slices use this
const rValidateSlice: (type: string, datum: Ref<stringMap>) => boolean = (
  type: string,
  datum: Ref<stringMap>,
) => {
  return !ValidateSlice(type, datum);
};

// TODO: enum warnings, don't show warning in some time after canceling
const ValidateSlice: (type: string, datum: Ref<stringMap>) => boolean = (
  type: string,
  datum: Ref<stringMap>,
) => {
  let uniqueType = uniqueDataType(datum);
  switch (type) {
    case "title":
      if (nonDeletedIter(data.value).next().value == datum.value) return true;
      // maybe use id?
      else {
        datum.value["type_change_warning"] =
          "Title can only be added at the first position, click the first add button and change new item to title";
        // BUG: yes, you can add multiple titles by doing so. 2 reasons not to prevent this, 1st is respect the choice of user
        return false;
      }
    case "time" || "place":
      return uniqueType(type);
    default:
      return true;
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
      <!-- TODO: why it says ResizeObserver loop completed with undelivered notifications. Maybe it's because display: none?! -->
      <button @click="datum.deleted = !datum.deleted">x</button>
      <!-- TODO: why cannot use v-model! -->
      <component
        :is="slice(datum.type)"
        :datum="datum"
        :servers="servers"
        :rValidator="rValidateSlice"
      />
      <i @click="delete datum.type_change_warning">{{
        datum.type_change_warning
      }}</i>
      <button @click="addItem(i + 1)">+</button>
      <!-- TODO: buttons to change type -->
      <!-- TODO: drag handle -->
    </div>
  </div>
</template>
