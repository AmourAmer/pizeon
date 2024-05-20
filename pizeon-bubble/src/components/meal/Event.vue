<script setup lang="ts">
import { Ref, computed } from "vue";
import { Repo } from "../../utils/type";
// FIXME: for sure
interface EventNotice {
  title: string;
  raw: string;
  avatar?: string;
}
const props = defineProps<{
  data: string;
  repo: Repo;
}>();
const cooked: Ref<EventNotice> = computed(() => {
  try {
    return JSON.parse(props.data);
  } catch {}

  return {
    title: "missing title still",
    raw: props.data,
  };
});
// TODO: deduplicate
const title = computed(() => cooked.value.title);
const avatar = computed(() => (cooked.value.avatar ? "O.o" : "o.O"));
const raw = computed(() => cooked.value.raw);
</script>

<template>
  <div>
    <h1>{{ title }}</h1>
    <p>{{ avatar }}</p>
    <!-- FIXME: don't nav at numbers -->
    <p
      contenteditable
      v-text="raw"
      style="text-align: left; white-space: pre-wrap"
    />
  </div>
</template>
