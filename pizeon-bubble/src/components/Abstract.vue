<script setup lang="ts">
import { computed } from "vue";
import time from "@utils/time";
import { Abstract } from "@utils/type";

const props = defineProps<Abstract>();
defineEmits<{
  (e: "check"): void;
}>();
const { month, day } = time(computed(() => props.date));
// TODO: should use useSwipe to move them
</script>

<template>
  <div @click="$emit('check')" v-if="date != 1">
    <h2 class="card-title">
      {{ title }}
    </h2>
    {{ body ? ": " + body : "" }} ~ {{ month }} / {{ day }}
  </div>
  <span class="loading loading-spinner" v-else />
</template>
