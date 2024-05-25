<script setup lang="ts">
import { computed } from "vue";
import time from "../utils/time";
interface Abstract {
  title: string;
  body?: string; // FIXME: seems notice.rs doesn't want Abstract to know body? Should send back the very first few letters.
  date: number;
}
const props = defineProps<Abstract>();
defineEmits<{
  (e: "check"): void;
}>();
const { month, day } = time(computed(() => props.date));
// TODO: should use useSwipe to move them
</script>

<template>
  <div @click="$emit('check')">
    <h2 class="card-title">
      {{ title }}
    </h2>
    {{ body ? ": " + body : "" }} ~ {{ month }} / {{ day }}
  </div>
</template>
