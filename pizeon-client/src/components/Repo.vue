<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref } from "vue";
import { asyncComputed } from "@vueuse/core";
import Bill from "./Bill.vue";
enum Repo {
  Fresh = "Fresh",
  Unwelcomed = "Unwelcomed",
  Fridge = "Fridge",
  Junk = "Junk",
}
const props = defineProps<{
  type: Repo;
}>();
const bill: Ref<string[] | null> = asyncComputed(
  async () =>
    await invoke("get_bill", {
      repo: props.type,
    }),
  null,
);
</script>

<template>
  <Bill :bill="bill || []" />
</template>
