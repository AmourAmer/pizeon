<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref } from "vue";
import { asyncComputed } from "@vueuse/core";
import Meals from "./Meals.vue";
enum Repo {
  Fresh = "Fresh",
  Unwelcomed = "Unwelcomed",
  Fridge = "Fridge",
  Junk = "Junk",
}
const props = defineProps<{
  type: Repo;
}>();
const ids: Ref<string[] | null> = asyncComputed(
  async () =>
    await invoke("get_ids", {
      repo: props.type,
    }),
  null,
);
</script>

<template>
  {{ ids }}
  <Meals :ids="ids || []" />
</template>
