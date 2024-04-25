<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref } from "vue";
import { asyncComputed, useStorage } from "@vueuse/core";
import Meal from "./Meal.vue";

interface Notice {
  date: number;
  heading: string;
  body: string;
}
// TODO these types are used anywhere, I should refactor
enum Repo {
  Fresh = "Fresh",
  Unwelcomed = "Unwelcomed",
  Fridge = "Fridge",
  Junk = "Junk",
}
type Signature = string;

const ids: Ref<string[]> = useStorage("mealIds", []);
async function getS(ids: string[]): Promise<[Notice, Signature[], Repo][]> {
  return Promise.all(
    ids.map(
      async (id) =>
        await invoke("get_notice", {
          id: id,
        }),
    ),
  );
}

const meals: Ref<[Notice, Signature[], Repo][] | []> = asyncComputed(
  // Should resolve one by one. Don't need to wait till all settle. Or should use cachedValues
  async () => await getS(ids.value),
  [],
);
</script>

<template>
  <div>
    <button @click="ids = []">Clear All</button>
    <!-- TODO scroll to btm, or use css to upside down? -->
    <Meal
      v-for="(meal, i) in meals"
      :repo="meal[2]"
      :key="i"
      :notice="meal[0]"
      :signs="meal[1]"
      @close="ids.splice(i, 1)"
    />
  </div>
</template>
