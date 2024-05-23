<script setup lang="ts">
import { Ref, ref, computed } from "vue";
import { toRefs, useTextareaAutosize } from "@vueuse/core";
import { stringMap } from "@utils/type";
import { useUpdateType } from "@utils/slice";

const props = defineProps<{
  destinations: string[];
  validator: (type: string, datum: Ref<stringMap>) => boolean;
}>();
const datum: Ref<stringMap> = defineModel("datum", { default: {} });

const textarea = ref();
useTextareaAutosize({
  element: textarea,
  input: toRefs(datum)["body"],
  styleProp: "minHeight",
});

useUpdateType(datum, ["body"], props.validator);

const {
  placeholderFn,
  rowsFn,
}: { placeholderFn: (datum: stringMap) => string; rowsFn?: () => number } =
  await import(`./${datum.value["type"]}.ts`); // TODO: error handling
const placeholder = computed(() => placeholderFn(props.destinations));
const rows = computed(rowsFn || (() => 3));
</script>

<template>
  <div>
    {{ datum.type.toUpperCase() }}:
    <!-- FIXME: how does https://vueuse.org/core/useTextareaAutosize/ impl this? -->
    <!-- FIXME: Ctrl-Z -->
    <textarea
      ref="textarea"
      class="resize-none"
      v-model="datum.body"
      :placeholder="placeholder"
      :rows="rows"
    />
  </div>
</template>
