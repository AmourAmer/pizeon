<script setup lang="ts">
import { stringMap } from "src/utils/type";
defineProps<{
  datum: stringMap;
}>();
</script>
<template>
  <span>
    {{ datum?.body }}
  </span>
</template>
