<script setup lang="ts">
const data = defineModel<(string[] | string)[]>();
defineProps<{
  server: String[];
}>();
</script>

<!-- TODO: server, formdatk -->
<template>
  <div>{{ server }}, {{ data }}</div>
</template>
