<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref } from "vue";
import { computedAsync, useStorage } from "@vueuse/core";

interface Abstract {
  heading: string;
  date: number;
}
enum Repo {
  Fresh = "Fresh",
  Unwelcomed = "Unwelcomed",
  Fridge = "Fridge",
  Junk = "Junk",
}

const props = defineProps<{ bill: string[] }>();
const ids: Ref<string[]> = useStorage("mealIds", []);

async function getS(bill: string[]): Promise<Abstract[]> {
  return Promise.all(
    bill.map(
      async (item) =>
        await invoke("get_abstract", {
          repo: Repo.Fresh,
          id: item,
        }),
    ),
  );
}

function addId(newId: string) {
  ids.value = ids.value.filter((id) => id != newId);
  ids.value.push(newId);
}

const abstracts: Ref<Abstract[] | null> = computedAsync(
  async () => await getS(props.bill),
  null,
);
</script>

<template>
  <div>
    <button v-for="(abstract, i) in abstracts" :key="i" @click="addId(bill[i])">
      {{ abstract }} --{{ bill[i] }}
    </button>
  </div>
</template>
