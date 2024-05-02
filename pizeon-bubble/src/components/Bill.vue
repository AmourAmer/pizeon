<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref } from "vue";
import { computedAsync, useStorage } from "@vueuse/core";
import Abstract from "./Abstract.vue";

interface Abstract {
  heading: string;
  body?: string;
  date: number;
}
enum Repo {
  Fresh = "Fresh",
  Blocked = "Blocked",
  Fridge = "Fridge",
  Junk = "Junk",
}

const props = defineProps<{ bill: string[] }>();
defineEmits<{
  (e: "changed"): void;
}>();
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

async function move(id: string, repo: Repo) {
  await invoke("move_notice", {
    id,
    repo,
  });
  // TODO force update abstracts
}
</script>

<template>
  <div v-for="(abstract, i) in abstracts" :key="i">
    <Abstract v-bind="abstract" @check="addId(bill[i])" />
    <button
      v-for="repo in Repo"
      @click="
        move(bill[i], repo);
        $emit('changed');
      "
    >
      TO {{ repo }}
    </button>
  </div>
</template>
