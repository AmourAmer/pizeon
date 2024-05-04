<script setup lang="ts">
import { computed } from "vue";
import time from "../utils/time";
interface Notice {
  date: number;
  heading: string;
  body: string;
}
type Signature = string;
enum Repo {
  Fresh = "Fresh",
  Blocked = "Blocked",
  Fridge = "Fridge",
  Junk = "Junk",
}

const props = defineProps<{
  notice: Notice;
  signs: Signature[];
  repo: Repo;
}>();
defineEmits<{
  (e: "close"): void;
}>();

const { month, day } = time(computed(() => props.notice.date));
</script>

<template>
  <div>
    <button @click="$emit('close')">Close me</button>
    <h1>{{ notice.heading }}</h1>
    <b>Repo: {{ repo }}</b>
    <p>{{ notice.body }}</p>
    <p>{{ month }}/{{ day }}</p>
    <s v-for="(sign, i) in signs" :key="i">{{ sign }}, </s>...
    <!-- <p>{{ id }}</p> -->
  </div>
</template>
