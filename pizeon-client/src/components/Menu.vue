<script setup lang="ts">
import { computed } from "vue";
enum Repo {
  Fresh = "Fresh",
  Unwelcomed = "Unwelcomed",
  Fridge = "Fridge",
  Junk = "Junk",
}
const props = defineProps<{
  type: Repo;
}>();
const msg = computed(() => {
  switch (props.type) {
    case "Fresh":
      return "Where you browse all notices";
    case "Junk":
      return "Your recently deleted notices";
    case "Unwelcomed":
      return "Recently blocked notices";
    case "Fridge":
      return "Your favorite notices, cautiously preserved, forever💕";
    default:
      break;
  }
});
</script>

<template>
  <div>{{ msg }}</div>
</template>
