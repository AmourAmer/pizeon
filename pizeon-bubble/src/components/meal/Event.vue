<script setup lang="ts">
import { Ref, computed } from "vue";
import { Repo, stringMap } from "src/utils/type";
import { useIngredientType } from "src/utils/ingredient";
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
    let parsed = JSON.parse(props.data);
    if (!Array.isArray(parsed.raw))
      parsed.raw = [{ type: "text", body: props.data }];
    return parsed;
  } catch {}

  return {
    title: "missing title still",
    raw: [{ type: "text", body: props.data }],
  };
});
const title = computed(() => cooked.value.title);
const raw = computed(() =>
  // Vue doesn't support (?.filter || (() => [])())
  cooked.value.raw.filter((datum) => datum.type != "title"),
);
</script>

<template>
  <div style="background: #ddd">
    <h1>{{ title }}</h1>
    <!-- FIXME: don't nav at numbers -->
    <component
      v-for="datum in raw"
      :is="useIngredientType(datum.type)"
      :datum="datum"
    >
    </component>
  </div>
</template>
