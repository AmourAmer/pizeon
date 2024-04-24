<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, Ref } from "vue";
import { asyncComputed } from "@vueuse/core";
import Meal from "./Meal.vue";

interface Notice {
  date: number;
  heading: string;
  body: string;
}
enum Repo {
  Fresh = "Fresh",
  Unwelcomed = "Unwelcomed",
  Fridge = "Fridge",
  Junk = "Junk",
}
type Signature = string;

const ids = ref(["1", "2"]);
async function getS(ids: string[]): Promise<[Notice, Signature[]][]> {
  return Promise.all(
    ids.map(
      async (id) =>
        await invoke("get_notice", {
          repo: Repo.Fresh,
          id: id,
        }),
    ),
  );
}

const meals: Ref<[Notice, Signature[]][] | null> = asyncComputed(
  // Should resolve one by one. Don't need to wait till all settle
  async () => await getS(ids.value),
  null,
);
</script>

<template>
  <div>
    <Meal
      v-for="(meal, i) in meals"
      :key="i"
      :notice="meal[0]"
      :signs="meal[1]"
    />
  </div>
</template>
