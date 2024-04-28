<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { computed, Ref } from "vue";
import { computedAsync, useStorage } from "@vueuse/core";
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

const mealTemplate: Meal = {
  notice: {
    date: 0,
    heading: "Please wait",
    body: "Fetching data",
  },
  signs: [],
  repo: Repo.Unwelcomed,
};

const ids: Ref<string[]> = useStorage("mealIds", []);
async function getNotice(id: string): Promise<Meal> {
  return await invoke("get_notice", {
    id: id,
  });
}

const meals: Ref<Ref<Meal>[]> = computed(() =>
  ids.value.map((id) =>
    computedAsync(async () => await getNotice(id), mealTemplate),
  ),
);
</script>

<template>
  <div>
    <button @click="ids = []">Clear All</button>
    <!-- TODO scroll to btm, or use css to upside down? -->
    {{ ids }}
    {{ meals }}
    <Meal
      v-for="(meal, i) in meals"
      :key="i"
      v-bind="meal.value"
      @close="ids.splice(i, 1)"
    />
  </div>
</template>
