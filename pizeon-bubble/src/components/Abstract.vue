<script setup lang="ts">
import { ref } from "vue";
import time from "@utils/time";
import { Abstract } from "@utils/type";

const props = defineProps<Abstract>();
const { month, day } = time(ref(props.date));
// TODO: should use useSwipe to move them
const timeout = ref("");
setTimeout(
  () =>
    (timeout.value =
      "Loading takes longer than expected, maybe there's an internal bug and this abstract can never been seen."),
  1000,
);
</script>

<template>
  <div v-if="date != 1">
    <h2 class="card-title">
      {{ title }}
    </h2>
    {{ body ? ": " + body : "" }} ~ {{ month }} / {{ day }}
  </div>
  <div v-else>
    <span class="loading loading-spinner" />
    {{ timeout }}
  </div>
</template>
