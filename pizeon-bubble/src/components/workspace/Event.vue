<script setup lang="ts">
// TODO: this is silly, I don't want to de-sync(though this doesn't happen). But if I don't do so, ts keeps warning me.
const data = defineModel<any[]>({ default: [] });
defineProps<{
  servers: String[];
}>();
</script>

<!-- TODO: server, formdatk -->
<template>
  <div>{{ servers }}, {{ data }}</div>
  <button @click="data.push(0)">+</button>
</template>
