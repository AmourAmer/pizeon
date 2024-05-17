<script setup lang="ts">
import { Ref } from "vue";
import { useStorage } from "@vueuse/core";
// TODO: this default is silly, I don't want to de-sync(though this doesn't happen). But if I don't do so, ts keeps warning me.
const data: Ref<string[]> = useStorage("event", []);
defineProps<{
  servers: String[];
}>();

defineExpose({
  // FIXME: someone help me name this plz orz
  foo() {
    return {
      raw: data.value,
    }; // TODO: heading, raw
  },
});
</script>

<!-- TODO: server, formdatk -->
<template>
  <div>{{ servers }}, {{ data }}</div>
  <button @click="data.push('0')">+</button>
</template>
