<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref, toRef } from "vue";
import { computedAsync } from "@vueuse/core";
import Bill from "./Bill.vue";
enum Repo {
  Fresh = "Fresh",
  Blocked = "Blocked",
  Fridge = "Fridge",
  Junk = "Junk",
}
const props = defineProps<{
  type: Repo;
}>();
// TODO Maybe should make Repos a frontend thing? At least should update when data change
const bill: Ref<string[]> = computedAsync(
  async () =>
    await invoke("get_bill", {
      repo: toRef(props.type).value,
    }),
  [],
);
</script>

<template>
  <Bill :bill="bill" />
</template>
