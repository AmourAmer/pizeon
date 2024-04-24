<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref } from "vue";
import { asyncComputed } from "@vueuse/core";

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

const abstracts: Ref<Abstract[] | null> = asyncComputed(
  async () => await getS(props.bill),
  null,
);
</script>

<template>
  <div>
    <p v-for="(abstract, i) in abstracts" :key="i">
      {{ abstract }} --{{ bill[i] }}
    </p>
  </div>
</template>
