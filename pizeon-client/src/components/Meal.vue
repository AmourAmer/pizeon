<script setup lang="ts">
import { computed } from "vue";
interface Notice {
  date: number;
  heading: string;
  body: string;
}
type Signature = string;
const props = defineProps<{
  id: string;
  notice: Notice;
  signs: Signature[];
}>();
defineEmits<{
  (e: "close", id: string): void;
}>();

const second = computed(() => (props.notice?.date || 0) * 1000);
const month = computed(() => new Date(second.value).getMonth() + 1);
const day = computed(() => new Date(second.value).getDate());
</script>

<template>
  <div>
    <button @click="$emit('close', id)">Close me</button>
    <h1>{{ notice?.heading }}</h1>
    <p>{{ notice?.body }}</p>
    <p>{{ month }}/{{ day }}</p>
    <s v-for="(sign, i) in signs" :key="i">{{ sign }}, </s>...
    <!-- <p>{{ id }}</p> -->
  </div>
</template>
