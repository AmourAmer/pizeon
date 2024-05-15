<script setup lang="ts">
import { Ref, computed } from "vue";
import { Repo } from "../../utils/type";
interface ClassicNotice {
  heading: string;
  raw: string;
}
const props = defineProps<{
  data: string;
  repo: Repo;
}>();
const cooked: Ref<ClassicNotice> = computed(() => {
  try {
    return JSON.parse(props.data);
  } catch {}

  return {
    heading: "missing heading still",
    raw: props.data,
  };
});
// TODO: deduplicate
const heading = computed(() => cooked.value.heading);
const raw = computed(() => cooked.value.raw);
</script>

<template>
  <div>
    <h1>{{ heading }}</h1>
    <!-- FIXME: don't nav at numbers -->
    <i
      contenteditable
      v-text="raw"
      style="text-align: left; white-space: pre-wrap"
    />
  </div>
</template>
