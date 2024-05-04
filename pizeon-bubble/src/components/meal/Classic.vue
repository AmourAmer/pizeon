<script setup lang="ts">
import { Ref, computed } from "vue";
interface ClassicNotice {
  heading: string;
  body: string;
}
const props = defineProps<{
  data: string;
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
