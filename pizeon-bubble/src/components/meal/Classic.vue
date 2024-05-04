<script setup lang="ts">
import { Ref, computed } from "vue";
interface ClassicNotice {
  heading: string;
  body: string;
}
const props = defineProps<{
  data: string;
}>();
const cooked: Ref<ClassicNotice> = computed(() => JSON.parse(props.data));
const heading = computed(() => cooked.value.heading);
const body = computed(() => cooked.value.body);
</script>

<template>
  <div>
    <h1>{{ heading }}</h1>
    <i>{{ body }}</i>
  </div>
</template>
