<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { computed, Ref } from "vue";
import { computedAsync, useStorage } from "@vueuse/core";
import Meal from "./Meal.vue";
import { Repo, Notice } from "../utils/type";

// TODO: these types are used anywhere, I should refactor

type Signature = string;
interface Meal {
  notice: Notice;
  signs: Signature[];
  repo: Repo;
}

const mealTemplate: Meal = {
  notice: {
    date: 0,
    title: "Please wait",
    bare_body: "Fetching data",
  },
  signs: [],
  repo: Repo.Blocked,
};

const ids: Ref<string[]> = useStorage("mealIds", []);
async function getNotice(id: string): Promise<Meal> {
  return await invoke("get_notice", {
    id: id,
  });
}

// TODO: intro id, :key
const meals: Ref<Ref<Meal>[]> = computed(() =>
  ids.value.map((id) =>
    computedAsync(async () => await getNotice(id), mealTemplate),
  ),
);
</script>

<template>
  <div>
    <button @click="ids = []">Clear All</button>
    <div style="display: flex; flex-direction: column-reverse">
      <Meal
        v-for="(meal, i) in meals"
        :key="i"
        v-bind="meal.value"
        @close="ids.splice(i, 1)"
      />
    </div>
  </div>
</template>
