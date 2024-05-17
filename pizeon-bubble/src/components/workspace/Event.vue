<script setup lang="ts">
import { Ref, computed } from "vue";
import { useStorage, useTextareaAutosize } from "@vueuse/core";
import { stringMap } from "../../utils/type";

const data: Ref<(string | stringMap)[]> = useStorage("event", []);
const props = defineProps<{
  servers: string[];
}>();

const { textarea } = useTextareaAutosize({ styleProp: "minHeight" });

defineExpose({
  finalize() {
    const result: { heading?: any; raw: (string | stringMap)[] } = {
      raw: data.value,
    }; // TODO: heading, raw
    if (typeof data.value[0] == "object" && data.value[0].type == "heading") {
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
</script>

<!-- TODO: server, formdatk -->
<template>
  <div>
    <div>{{ servers }}, {{ data }}</div>
    <button @click="data.push('0')">+</button>
    <textarea
      ref="textarea"
      class="resize-none"
      :placeholder="placeholder"
      :rows="3"
    />
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
