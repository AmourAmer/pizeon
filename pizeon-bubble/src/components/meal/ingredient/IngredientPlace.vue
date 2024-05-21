<script setup lang="ts">
import { stringMap } from "src/utils/type";
defineProps<{
  datum: stringMap;
}>();
</script>
<template>
  <div>
    PLACE: <i>{{ datum?.body }}</i>
  </div>
</template>
