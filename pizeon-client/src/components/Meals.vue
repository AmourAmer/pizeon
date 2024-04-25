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

const ids: Ref<string[]> = useStorage("mealIds", []);
async function getNotice(id: string): Promise<[Notice, Signature[], Repo]> {
  return await invoke("get_notice", {
    id: id,
  });
}

// TODO use v-bind obj to simplify
const meals: Ref<Ref<[Notice, Signature[], Repo] | null>[]> = ref(
  ids.value.map((id) =>
    asyncComputed(
      // Should resolve one by one. Don't need to wait till all settle. Or should use cachedValues
      async () => await getNotice(id),
      ["1m", ["2"], "2"],
    ),
  ),
);
</script>

<template>
  <div>
    <button @click="ids = []">Clear All</button>
    <!-- TODO scroll to btm, or use css to upside down? -->
    <div v-for="(meal, i) in meals" :key="i">
      {{ meal }}
      <Meal
        v-if="meal != null"
        :notice="meal[0]"
        :signs="meal[1]"
        :repo="meal[2]"
        @close="ids.splice(i, 1)"
      />
    </div>
  </div>
</template>
