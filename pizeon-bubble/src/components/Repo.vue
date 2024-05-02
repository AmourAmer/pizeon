<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref } from "vue";
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
// TODO: Maybe should make Repos a frontend thing? At least should update when data change
const f = () =>
  invoke("get_bill", {
    repo: props.type,
  }) as Promise<string[]>;
const bill: Ref<string[]> = computedAsync(async () => await f(), []);
const update = () => f().then((res) => (bill.value = res));
</script>

<template>
  BILL {{ bill }}
  <Bill :bill="bill" @changed="update" />
</template>
