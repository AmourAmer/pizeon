<script setup lang="ts">
import { stringMap } from "src/utils/type";
defineProps<{
  datum: stringMap;
}>();
</script>
<template>
  <!-- Of course, it's silly to show title again -->
  <!-- div>
    TITLE: <i>{{ datum?.body }}</i>
  </div -->
</template>
