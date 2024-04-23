<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, computed } from "vue";
const notice = ref("");
async function get() {
  notice.value = await invoke("get_notice");
}
const second = computed(() => notice.value.date * 1000);
const month = computed(() => new Date(second.value).getMonth() + 1);
const day = computed(() => new Date(second.value).getDate());
get();
</script>

<template>
  <div>
    <h1>{{ notice.heading }}</h1>
    <p>{{ notice.body }}</p>
    <p>{{ month }}/{{ day }}</p>
  </div>
</template>
