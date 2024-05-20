<script setup lang="ts">
import { Ref, computed } from "vue";
import { Repo, stringMap } from "src/utils/type";
interface EventNotice {
  title: string;
  raw: stringMap[];
}
const props = defineProps<{
  data: string;
  repo?: Repo;
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
const title = computed(() => cooked.value.title);
// const avatar = computed(() => (cooked.value.avatar ? "O.o" : "o.O"));
const raw = computed(() => cooked.value.raw);
console.log(cooked.value, raw.value);
</script>

<template>
  <div style="background: #ddd">
    <h1>{{ title }}</h1>
    <!-- FIXME: don't nav at numbers -->
    <p v-for="datum in raw">{{ datum.type }}</p>
  </div>
</template>
