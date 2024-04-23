<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
const notice = ref("");
async function get() {
  notice.value = await invoke("get_notice");
}
get();
</script>

<template>
  <div>
    <h1 @click="get">{{ notice.heading }}</h1>
  </div>
</template>
