<script setup lang="ts">
import { computed } from "vue";
const props = defineProps({
  type: String,
});
const msg = computed(() => {
  switch (props.type) {
    case "menu":
      return "Where you browse all notices";
    case "junk":
      return "Your recently deleted notices";
    case "unwelcomed":
      return "Recently blocked notices";
    case "fridge":
      return "Your favorite notices, cautiously preserved, forever💕";
    default:
      break;
  }
});
</script>

<template>
  <div>{{ msg }}</div>
</template>
