<script setup lang="ts">
import { stringMap } from "src/utils/type";
defineProps<{
  datum: stringMap;
}>();
</script>
<template>
  <div>
    TITLE: <i>{{ datum?.body }}</i>
  </div>
</template>
