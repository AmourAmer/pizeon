<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref, computed } from "vue";
import { computedAsync, useLocalStorage } from "@vueuse/core";
import Abstract from "./Abstract.vue";
import { Repo, AbstractType } from "@utils/type";

const props = defineProps<{ bill: string[] }>();
const emit = defineEmits<{
  (e: "changed"): void;
}>();
const ids: Ref<string[]> = useLocalStorage("mealIds", []);

function getS(bill: string[]): Promise<AbstractType>[] {
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
// TODO: load a few at 1st and load more on scroll
const abstracts: Ref<Ref<AbstractType>[]> = computed(() =>
  getS(props.bill).map((ab) =>
    computedAsync(async () => await ab, {
      title: "",
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
  <!-- TODO: change to id, fridge & trash still flashes, while fresh doesn't -->
  <div>
    <!-- TODO: scale on hover? That's should be fun. Though hover is not that useful especially for mobile devices -->
    <div
      v-for="(abstract, i) in abstracts"
      :key="abstract.value.date"
      class="card bg-neutral text-neutral-content hover:bg-violet-300"
      :class="abstract.value.class || 'glass'"
      :data-theme="'dracula'"
    >
      <div class="card-body items-center text-center">
        <Abstract v-bind="abstract.value" @check="addId(bill[i])" />
        <div class="card-actions justify-end">
          <button
            v-for="repo in Repo"
            v-show="repo != 'Blocked'"
            @click="move(bill[i], repo)"
            class="btn btn-primary"
          >
            TO {{ repo }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
