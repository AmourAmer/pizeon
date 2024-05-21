<script setup lang="ts">
import { stringMap } from "src/utils/type";
defineProps<{
  datum: stringMap;
}>();
</script>
<template>
  <div>
    Host: <b>{{ datum?.name }}</b>
    <p>Avatar: O.o</p>
    <i>{{ datum?.title }}</i>
    <s>{{ datum?.description }}</s>
  </div>
</template>
