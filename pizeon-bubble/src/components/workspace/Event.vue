<script setup lang="ts">
import { Ref } from "vue";
import { useStorage } from "@vueuse/core";
import { stringMap } from "../../utils/type";

const data: Ref<(string | stringMap)[]> = useStorage("event", []);
defineProps<{
  servers: String[];
}>();

defineExpose({
  // FIXME: someone help me name this plz orz
  foo() {
    const bar: { heading?: any; raw: (string | stringMap)[] } = {
      raw: data.value,
    }; // TODO: heading, raw
    if (typeof data.value[0] == "object" && data.value[0].type == "heading") {
      bar.heading = data.value[0].body;
    }
    return bar;
  },
});
</script>

<!-- TODO: server, formdatk -->
<template>
  <div>{{ servers }}, {{ data }}</div>
  <button @click="data.push('0')">+</button>
</template>
