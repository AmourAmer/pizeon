<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref } from "vue";
import { computedAsync } from "@vueuse/core";
import Bill from "./Bill.vue";
import { Repo } from "../utils/type";
const props = defineProps<{
  type: Repo;
}>();
// TODO: A not-so-elegant impl of updating bill. Should let back-end tell when updated successfully
const f = () =>
  invoke("get_bill", {
    repo: props.type,
  }) as Promise<string[]>;
const bill: Ref<string[]> = computedAsync(async () => await f(), []);
const update = (id: string, repo: Repo) => {
  if (repo != props.type) {
    bill.value = bill.value.filter((i) => i != id);
  }
};
</script>

<template>
  <div>
    <Bill :bill="bill" @changed="update" />
  </div>
</template>
