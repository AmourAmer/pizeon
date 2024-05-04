<script setup lang="ts">
import { computed } from "vue";
import time from "../utils/time";
import { Notice } from "../utils/type";
import ClassicNotice from "./meal/Classic.vue";

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
const noticeTemplate = computed(() => {
  try {
    return JSON.parse(props.notice.bare_body)?.template.toLowerCase();
  } catch {
    return "classic";
  }
});
</script>

<template>
  <div style="box-shadow: 0 8px 8px rgba(0, 0, 0, 0.5); margin: 8px">
    <button @click="$emit('close')">Close me</button>
    <b>Repo: {{ repo }}</b>
    <ClassicNotice
      v-if="noticeTemplate == 'classic'"
      :data="notice.bare_body"
    />
    <p>{{ month }}/{{ day }}</p>
    <s v-for="(sign, i) in signs" :key="i">{{ sign }}, </s>...
  </div>
</template>
