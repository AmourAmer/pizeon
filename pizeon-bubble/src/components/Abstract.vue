<script setup lang="ts">
import { toRef } from "vue";
import time from "../utils/time";
interface Abstract {
  heading: string;
  body?: string; // FIXME: seems notice.rs doesn't want Abstract to know body? Should send back the very first few letters.
  date: number;
}
const props = defineProps<Abstract>();
defineEmits<{
  (e: "check"): void;
}>();
const { month, day } = time(toRef(props.date).value);
// TODO: should use useSwipe to move them
</script>

<template>
  <div>
    <button @click="$emit('check')">
      {{ heading }} {{ body ? ": " + body : "" }} ~ {{ month }} / {{ day }}
    </button>
  </div>
</template>
