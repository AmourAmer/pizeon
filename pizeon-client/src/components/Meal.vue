<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, computed, Ref } from "vue";
interface Notice {
  date: number;
  heading: string;
  body: string;
}
const props = defineProps({
  id: String,
});

const notice: Ref<Notice | undefined> = ref();
const signs: Ref<string[]> = ref([]);
async function get() {
  const res: [Notice, string[]] = await invoke("get_notice");
  notice.value = res[0];
  signs.value = res[1];
}
const second = computed(() => (notice.value?.date || 0) * 1000);
const month = computed(() => new Date(second.value).getMonth() + 1);
const day = computed(() => new Date(second.value).getDate());
get();
</script>

<template>
  <div>
    <h1>{{ notice?.heading }}</h1>
    <p>{{ notice?.body }}</p>
    <p>{{ month }}/{{ day }}</p>
    <s v-for="(sign, i) in signs" :key="i">{{ sign }}</s>
    <p>{{ id }}</p>
  </div>
</template>
