<!-- WARNING: Deprecated, will be removed in the future. No one should use this, ~~except me~~ -->
<script setup lang="ts">
import { Ref, computed } from "vue";
import { Repo } from "../../utils/type";
interface ClassicNotice {
  title: string;
  raw: string;
}
const props = defineProps<{
  data: string;
  repo: Repo;
}>();
const cooked: Ref<ClassicNotice> = computed(() => {
  try {
    return JSON.parse(props.data);
  } catch {}

  return {
    title: "missing title still",
    raw: props.data,
  };
});
// TODO: deduplicate
const title = computed(() => cooked.value.title);
const raw = computed(() => cooked.value.raw);
</script>

<template>
  <div>
    <b
      >You are not supposed to see this, this means some one creates a notice
      with a type being deprecated. If you know the sender, please tell him be
      cautious about this and pay attention to changelog, which might exist</b
    >
    <h1>{{ title }}</h1>
    <!-- FIXME: don't nav at numbers -->
    <i
      contenteditable
      v-text="raw"
      style="text-align: left; white-space: pre-wrap"
    />
  </div>
</template>
