<script setup lang="ts">
const model = defineModel<(string[] | string)[]>();
</script>

<!-- TODO: server, formdatk -->
<template>
  <div>{{ model }}</div>
</template>
