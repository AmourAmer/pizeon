<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref } from "vue";
import { computedAsync, useStorage } from "@vueuse/core";
import Abstract from "./Abstract.vue";
import { Repo } from "../utils/type";

interface Abstract {
  heading: string;
  body?: string;
  date: number;
}

const props = defineProps<{ bill: string[] }>();
const emit = defineEmits<{
  (e: "changed"): void;
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

// TODO: introduce id, :key="id"
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
  // TODO: races?
  emit("changed");
}
</script>

<template>
  <!-- TODO: change to id -->
  <div
    v-for="(abstract, i) in abstracts"
    :key="abstract.value.date"
    style="box-shadow: 0 8px 8px rgba(128, 0, 128, 0.5); margin: 48px"
  >
    {{ abstract }}
    <Abstract v-bind="abstract.value" @check="addId(bill[i])" />
    <button
      v-for="repo in Repo"
      v-show="repo != 'Blocked'"
      @click="move(bill[i], repo)"
    >
      TO {{ repo }}
    </button>
  </div>
</template>
