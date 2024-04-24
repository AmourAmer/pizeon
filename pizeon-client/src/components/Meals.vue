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
type Signature = string;

const ids: Ref<string[]> = useStorage("mealIds", []);
async function getS(ids: string[]): Promise<[Notice, Signature[]][]> {
  return Promise.all(
    ids.map(
      async (id) =>
        await invoke("get_notice", {
          id: id,
        }),
    ),
  );
}

const meals: Ref<[Notice, Signature[]][] | null> = asyncComputed(
  // Should resolve one by one. Don't need to wait till all settle. Or should use cachedValues
  async () => await getS(ids.value),
  null,
);
</script>

<template>
  <div>
    <Meal
      v-for="(meal, i) in meals?.reverse()"
      :key="i"
      :notice="meal[0]"
      :signs="meal[1]"
    />
  </div>
</template>
