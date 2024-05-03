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
  (e: "changed", id: string, repo: Repo): void;
}>();
const ids: Ref<string[]> = useStorage("mealIds", []);

function getS(bill: string[]): Promise<Abstract>[] {
  return bill.map((item) =>
    invoke("get_abstract", {
      id: item,
    }),
  );
}

function addId(newId: string) {
  ids.value = ids.value.filter((id) => id != newId);
  ids.value.push(newId);
}

const abstracts: Ref<Ref<Abstract>[]> = computedAsync(async () =>
  getS(props.bill).map((ab) =>
    computedAsync(async () => await ab, {
      heading: "fetch or parse failed, consider deleting it?",
      date: 1,
    }),
  ),
);

async function move(id: string, repo: Repo) {
  await invoke("move_notice", {
    id,
    repo,
  });
  // TODO: force update abstracts
}
</script>

<template>
  <div v-for="(abstract, i) in abstracts" :key="i">
    {{ abstract }}
    <Abstract v-bind="abstract.value" @check="addId(bill[i])" />
    <button
      v-for="repo in Repo"
      @click="
        move(bill[i], repo);
        $emit('changed', bill[i], repo);
      "
    >
      TO {{ repo }}
    </button>
  </div>
</template>
