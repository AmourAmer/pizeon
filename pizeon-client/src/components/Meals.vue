<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref } from "vue";
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

const meals: Ref<[Notice, Signature[]][] | null> = asyncComputed(
  async () => [
    // FUCK! I can't rewrite it in function! Why?!
    await invoke("get_notice", {
      repo: Repo.Fresh,
      id: "1",
    }),
  ],
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
