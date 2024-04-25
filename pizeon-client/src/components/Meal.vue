<script setup lang="ts">
import { toRef, computed } from "vue";
interface Notice {
  date: number;
  heading: string;
  body: string;
}
type Signature = string;
enum Repo {
  Fresh = "Fresh",
  Unwelcomed = "Unwelcomed",
  Fridge = "Fridge",
  Junk = "Junk",
}

const props = defineProps<{
  notice: Notice;
  signs: Signature[];
  repo: Repo;
}>();
const notice1 = toRef(props.notice);
defineEmits<{
  (e: "close"): void;
}>();

const second = computed(() => (props.notice?.date || 0) * 1000);
const month = computed(() => new Date(second.value).getMonth() + 1);
const day = computed(() => new Date(second.value).getDate());
</script>

<template>
  <div>
    Notice1: {{ notice1 }} Notice: {{ notice }} Sings: {{ signs }}
    <button @click="$emit('close')">Close me</button>
    <h1>{{ notice?.heading }}</h1>
    <p>{{ notice?.body }}</p>
    <p>{{ month }}/{{ day }}</p>
    <s v-for="(sign, i) in signs" :key="i">{{ sign }}, </s>...
    <!-- <p>{{ id }}</p> -->
  </div>
</template>
