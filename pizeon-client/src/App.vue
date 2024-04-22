<script setup lang="ts">
import { reactive } from "vue";

const notice = reactive({
  heading: "Join us",
  body: "Don't ask",
  date: new Date(),
});
import Notice from "./components/Notice.vue";
</script>

<template>
  <div class="container">
    <h1>Welcome to Tauri!</h1>
    <Notice :data="notice"/>
  </div>
</template>


