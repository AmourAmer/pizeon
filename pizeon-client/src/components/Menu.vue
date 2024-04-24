<script setup lang="ts">
defineProps<{ menu: string[] }>();
</script>

<template>
  <div>
    <p v-for="(item, i) in menu" :key="i">{{ item }}</p>
  </div>
</template>
