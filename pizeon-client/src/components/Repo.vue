<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref } from "vue";
import { asyncComputed } from "@vueuse/core";
import Menu from "./Menu.vue";
enum Repo {
  Fresh = "Fresh",
  Unwelcomed = "Unwelcomed",
  Fridge = "Fridge",
  Junk = "Junk",
}
const props = defineProps<{
  type: Repo;
}>();
const repo: Ref<string[] | null> = asyncComputed(
  async () =>
    await invoke("get_repo", {
      repo: props.type,
    }),
  null,
);
</script>

<template>
  <Menu :menu="repo || []" />
</template>
