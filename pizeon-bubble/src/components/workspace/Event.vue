<script setup lang="ts">
import { Ref, computed } from "vue";
import { useStorage, useTextareaAutosize } from "@vueuse/core";
import { stringMap } from "../../utils/type";
import { v4 as uuidv4 } from "uuid";

const data: Ref<stringMap[]> = useStorage("event", []);
const props = defineProps<{
  servers: string[];
}>();

const { textarea } = useTextareaAutosize({ styleProp: "minHeight" });

defineExpose({
  finalize() {
    const result: { heading?: any; raw: stringMap[] } = {
      raw: data.value.filter((item) => delete item.symbol),
    };
    if (data.value[0].type == "heading") {
      result.heading = data.value[0].body;
    }
    return result;
  },
});

const placeholder = computed(() => {
  const msg = (dest: string) =>
    "What notice do you want to send on " + dest + "?";
  switch (props.servers.length) {
    case 0:
      return "Please choose a server to send notice to";
    case 1:
      return msg(props.servers[0]);
    case 2:
      return msg(props.servers[0] + " and " + props.servers[1]);
    default:
      return msg(
        props.servers.slice(0, -1).join(", ") +
          ", and " +
          props.servers.slice(-1),
      );
  }
});

const addItem = (idx: number) => {
  data.value.splice(idx, 0, {
    symbol: uuidv4(),
    type: "text",
  });
};
</script>

<template>
  <div>
    <div>{{ servers }}, {{ data }}</div>
    <!-- <textarea ref="textarea" class="resize-none" :placeholder="placeholder" :rows="3" /> -->
    <button @click="addItem(0)">+</button>
    <div v-for="(datum, i) in data" :key="datum.symbol">
      {{ datum.symbol }}
      <button @click="data.splice(i, 1)">x</button>
      {{ datum }}
      <button @click="addItem(i + 1)">+</button>
    </div>
  </div>
</template>

<style scoped>
textarea {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

textarea::-webkit-scrollbar {
  display: none;
}
</style>
