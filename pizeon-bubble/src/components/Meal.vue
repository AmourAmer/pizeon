<script setup lang="ts">
import { computed, Ref } from "vue";
import time from "src/utils/time";
import { Repo, Notice } from "../utils/type";
import ClassicNotice from "./meal/Classic.vue";
import EventNotice from "./meal/Event.vue";

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
    // WARNING: should deprecate
    return "classic";
  }
});
const noticeComponent = computed(() => {
  switch (noticeTemplate.value) {
    // WARNING: should deprecate
    case "classic":
      return ClassicNotice;
    case "event":
      return EventNotice;
    default:
      return EventNotice;
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
    <component :is="noticeComponent" :data="notice.bare_body" :repo="repo" />
    <p>{{ month }}/{{ day }}</p>
    <s v-for="sign in signs" :key="sign">{{ sign }}, </s>...
  </div>
</template>
