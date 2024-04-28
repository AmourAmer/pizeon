<script setup lang="ts">
import { ref } from "vue";
import time from "../utils/time";
interface Abstract {
  heading: string;
  body?: string;
  date: number;
}
const props = defineProps<Abstract>();
defineEmits<{
  (e: "check"): void;
}>();
const { month, day } = time(ref(props.date));
// TODO should use useSwipe to move them
</script>

<template>
  <div>
    <button @click="$emit('check')">
      {{ heading }} {{ body ? ": " + body : "" }} ~ {{ month }} / {{ day }}
    </button>
  </div>
</template>
