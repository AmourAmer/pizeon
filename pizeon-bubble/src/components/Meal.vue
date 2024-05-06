<script setup lang="ts">
import { computed, Ref } from "vue";
import time from "../utils/time";
import { Repo, Notice } from "../utils/type";
import ClassicNotice from "./meal/Classic.vue";

type Signature = string;

const props = defineProps<{
  notice: Notice;
  repo: Repo;
}>();
defineEmits<{
  (e: "close"): void;
}>();

// TODO: any need to toRef props? maybe should use toRefs to be more elegant
const { month, day } = time(computed(() => props.notice.date));
const noticeTemplate = computed(() => {
  try {
    return JSON.parse(props.notice.bare_body)?.template.toLowerCase();
  } catch {
    return "classic";
  }
});
const signs: Ref<Signature[]> = computed(() => {
  try {
    return JSON.parse(props.notice.bare_body)?.signature || [];
  } catch {
    return [];
  }
});
</script>

<template>
  <!-- TODO: 奏折/悬赏/皇榜/魔鬼契约/流云/羊皮 -->
  <div style="box-shadow: 0 8px 8px rgba(0, 0, 0, 0.5); margin: 8px">
    <button @click="$emit('close')">Close me</button>
    <b>Repo: {{ repo }}</b>
    <ClassicNotice
      v-if="noticeTemplate == 'classic'"
      :data="notice.bare_body"
      :repo="repo"
    />
    <p>{{ month }}/{{ day }}</p>
    <s v-for="(sign, i) in signs" :key="i">{{ sign }}, </s>...
  </div>
</template>
