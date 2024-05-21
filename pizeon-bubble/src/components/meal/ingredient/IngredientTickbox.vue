<script setup lang="ts">
import { stringMap } from "src/utils/type";
defineProps<{
  datum: stringMap;
}>();
</script>
<template>
  <div>
    <input type="radio" />
    <i>{{ datum?.body }}</i>
  </div>
</template>
