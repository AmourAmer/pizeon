<script setup lang="ts">
import { Ref } from "vue";
import { stringMap } from "@utils/type";
import { useData } from "src/utils/draft";
import { useSliceType, useNextSliceType, useFinalize } from "src/utils/slice";
import { v4 as uuidv4 } from "uuid";

// TODO: draggable, not so urgent
// TODO: another storage name
const { data, nonDeletedIter, uniqueDataType } = useData("event");
if (!data.value.length) pushInitTemplate();
defineProps<{
  destinations: string[];
}>();

function finalize() {
  let nonDeletedData = [...nonDeletedIter(data.value)];
  const id = (e: any) => e;
  const result: { title?: string; raw: stringMap[] } = {
    raw: nonDeletedData
      .map((datum) => useFinalize(datum))
      .filter(id) as stringMap[],
  };
  if (nonDeletedData[0]?.type == "title") {
    result.title = nonDeletedData[0].body;
  }
  return result;
}
defineExpose({
  finalize() {
    // TODO: option to keep
    const result = finalize();
    {
      data.value = [];
      pushInitTemplate();
    }
    return result;
  },
  preview() {
    return JSON.stringify(finalize());
  },
});

function pushInitTemplate() {
  addItem("title");
  addItem("host");
  addItem("time");
  addItem("place");
  addItem();
}

const addItem = (type = "text", idx = data.value.length) => {
  // TODO: validate type, use exported dict
  data.value.splice(idx, 0, {
    id: uuidv4(),
    type,
  });
};

const slice = useSliceType();
const nextSliceType = useNextSliceType();

// TODO: enum warnings, don't show warning in some time after canceling
const validateSlice: (type: string, datum: Ref<stringMap>) => boolean = (
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
    case "time":
    case "place":
      return uniqueType(type);
    default:
      return true;
  }
};
</script>

<template>
  <div>
    <button @click="addItem('text', 0)">+</button>
    {{ data }}
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
      <Suspense>
        <component
          :is="slice(datum.type)"
          :datum="datum"
          :destinations="destinations"
          :validator="validateSlice"
          v-show="!datum.deleted"
        />
      </Suspense>
      <i @click="delete datum.type_change_warning">{{
        datum.type_change_warning
      }}</i>
      <button @click="addItem(nextSliceType(datum.type), i + 1)">+</button>
      <!-- TODO: buttons to change type -->
      <!-- TODO: drag handle -->
    </div>
  </div>
</template>
