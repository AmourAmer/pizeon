<script setup lang="ts">
import { Ref, computed } from "vue";
import { Repo } from "../../utils/type";
interface ClassicNotice {
  heading: string;
  body: string;
  signature: string[];
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
      signature: [],
    };
  }
});
const heading = computed(() => cooked.value.heading);
const body = computed(() => cooked.value.body);
const signature = computed(() => cooked.value.signature || []);
</script>

<template>
  <div>
    <h1>{{ heading }}</h1>
    <i>{{ body }}</i>
    <b>{{ signature }}</b>
  </div>
</template>
