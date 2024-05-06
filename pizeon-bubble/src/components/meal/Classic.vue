<script setup lang="ts">
import { Ref, computed } from "vue";
import { Repo } from "../../utils/type";
interface ClassicNotice {
  heading: string;
  body: string;
}
const props = defineProps<{
  data: string;
  repo: Repo;
}>();
const cooked: Ref<ClassicNotice> = computed(() => {
  try {
    return JSON.parse(props.data);
  } catch {
    return {
      heading: "missing heading still",
      body: props.data,
    };
  }
});
const heading = computed(() => cooked.value.heading);
const body = computed(() => cooked.value.body);
</script>

<template>
  <div>
    <h1>{{ heading }}</h1>
    <i>{{ body }}</i>
  </div>
</template>
