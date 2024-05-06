<script setup lang="ts">
import { useStorage } from "@vueuse/core";

const expert_mode = useStorage("expert_mode", false);
</script>

<template>
  <div>
    <p v-if="expert_mode">
      Expert Mode is currently unavailable, wait for months and update your app
      to see if I've implemented that
    </p>
    <h1>Who are you</h1>
    <h1>Who you trust</h1>
    <h1>What you do</h1>
  </div>
</template>
