<script setup lang="ts">
import { ref } from "vue";
import time from "../utils/time";
interface Abstract {
  heading: string;
  body?: string;
  date: number;
}
enum Repo {
  Fresh = "Fresh",
  Unwelcomed = "Unwelcomed",
  Fridge = "Fridge",
  Junk = "Junk",
}
const props = defineProps<Abstract>();
defineEmits<{
  (check: Event): void;
}>();
const { month, day } = time(ref(props.date));
// TODO should use useSwipe to move them
</script>

<template>
  <div>
    <button @click="$emit('check')">
      {{ heading }} {{ body ? ": " + body : "" }} ~ {{ month }} / {{ day }}
    </button>
    <button v-for="repo in Repo">TO {{ repo }}</button>
  </div>
</template>
