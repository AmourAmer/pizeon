<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, Ref } from "vue";
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
interface Meal {
  notice: Notice;
  signs: Signature[];
  repo: Repo;
}

const ids: Ref<string[]> = useStorage("mealIds", []);
async function getNotice(id: string): Promise<Meal> {
  return await invoke("get_notice", {
    id: id,
  });
}

const meals: Ref<Ref<Meal | null>[]> = ref(
  ids.value.map((id) =>
    asyncComputed(
      // Should use cachedValues
      async () => await getNotice(id),
      null,
    ),
  ),
);
</script>

<template>
  <div>
    <button @click="ids = []">Clear All</button>
    <!-- TODO scroll to btm, or use css to upside down? -->
    <div v-for="(meal, i) in meals" :key="i">
      MEAL: {{ meal }}
      <Meal v-bind="meal" @close="ids.splice(i, 1)" />
    </div>
  </div>
</template>
