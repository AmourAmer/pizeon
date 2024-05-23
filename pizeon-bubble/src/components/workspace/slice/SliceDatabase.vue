<script setup lang="ts">
import { Ref, computed } from "vue";
import { useTextareaAutosize } from "@vueuse/core";
import { stringMap } from "@utils/type";
import { useUpdateType, useBindDatum } from "@utils/slice";

const { textarea, input } = useTextareaAutosize({ styleProp: "minHeight" });

const props = defineProps<{
  rValidator: (type: string, datum: Ref<stringMap>) => boolean;
}>();
const datum: Ref<stringMap> = defineModel("datum", { default: {} });
useBindDatum(datum, { body: input });
const db = computed(() => {
  try {
    return input.value.split("\n").map((it) => it.split("\t"));
  } catch {
    return [];
  }
});

useUpdateType(datum, { body: input }, (...args) => !props.rValidator(...args));

const placeholder = computed(() => "paste table containing needed info here");
</script>

<template>
  <div>
    <!-- FIXME: how does https://vueuse.org/core/useTextareaAutosize/ impl this? -->
    <textarea
      ref="textarea"
      class="resize-none"
      v-model="input"
      :placeholder="placeholder"
      :rows="5"
    />
    {{ db }}
  </div>
</template>
