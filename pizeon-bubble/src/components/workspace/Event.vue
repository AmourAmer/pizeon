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
  if (nonDeletedData[0]?.type == "title") result.title = nonDeletedData[0].body;
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
        clearTimeout(datum.value["type_change_warning_timeout"]);
        datum.value["type_change_warning_timeout"] = setTimeout(
          () => delete datum.value["type_change_warning"],
          3000,
        );
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
    <!-- TODO: the radius should be smaller -->
    <div v-for="(datum, i) in data" :key="datum.id" class="m-2">
      <div class="flex justify-end">
        <!-- TODO: buttons to change type -->
        <!-- FIXME: This icon doesn't grow on zoom -->
        <div
          @click="addItem(nextSliceType(datum.type), i + 1)"
          class="btn btn-circle btn-xs mx-2"
        >
          <span class="scale-[2.5]">+</span>
        </div>
        <!-- TODO: correct svg size -->
        <label class="btn btn-circle btn-xs swap swap-rotate">
          <!-- this hidden checkbox controls the state -->
          <input type="checkbox" v-model="datum.deleted" class="peer" />

          <!-- hamburger icon -->
          <svg
            class="swap-on fill-current"
            xmlns="http://www.w3.org/2000/svg"
            width="32"
            height="32"
            viewBox="0 0 512 512"
          >
            <path
              d="M64,384H448V341.33H64Zm0-106.67H448V234.67H64ZM64,128v42.67H448V128Z"
            />
          </svg>

          <!-- close icon -->
          <svg
            class="swap-off fill-current"
            xmlns="http://www.w3.org/2000/svg"
            width="32"
            height="32"
            viewBox="0 0 512 512"
          >
            <polygon
              points="400 145.49 366.51 112 256 222.51 145.49 112 112 145.49 222.51 256 112 366.51 145.49 400 256 289.49 366.51 400 400 366.51 289.49 256 400 145.49"
            />
          </svg>
        </label>
      </div>
      <div v-show="!datum.deleted">
        <Suspense>
          <component
            :is="slice(datum.type)"
            :datum="datum"
            :destinations="destinations"
            :validator="validateSlice"
          />
        </Suspense>
        <!-- TODO: transition -->
        <div
          role="alert"
          class="alert alert-warning"
          @click="delete datum.type_change_warning"
          v-if="datum.type_change_warning"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="stroke-current shrink-0 h-6 w-6"
            fill="none"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
            />
          </svg>
          <span>{{ datum.type_change_warning }}</span>
        </div>
      </div>
      <!-- TODO: drag handle -->
    </div>
  </div>
</template>
