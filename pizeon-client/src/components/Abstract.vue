<script setup lang="ts">
interface Abstract {
  heading: string;
  body?: string;
  date: number;
}
defineProps<Abstract>();
// TODO should use useSwipe to move them
</script>

<template>
  <div>
    <button>{{ heading }} {{ body ? ": " + body : "" }} / {{ date }}</button>
  </div>
</template>
