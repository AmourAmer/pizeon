<script setup lang="ts">
import { Ref, ref, computed } from "vue";
import { computedAsync, toRefs, useTextareaAutosize } from "@vueuse/core";
import { stringMap } from "@utils/type";
import { useUpdateType } from "@utils/slice";
import { ToRefs } from "vue";

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
}: ToRefs<{
  placeholderFn: (datum: stringMap) => string;
  rowsFn?: () => number;
}> = toRefs(
  computedAsync(async () => await import(`./${datum.value["type"]}.ts`), {
    placeholderFn: () => "",
    rowsFn: () => 9, // WARNING: can't drop this, ?
  }),
); // TODO: error handling
const placeholder = computed(() => placeholderFn.value(props.destinations));
const rows = computed(() => (rowsFn?.value || (() => 3))()); // FIXME: this is little bit strange, and it misses a tick shrinking

// FIXME: this solution is not good enough
const vFocus = {
  mounted: (el: HTMLElement) => el.focus(),
};
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
      v-focus
    />
  </div>
</template>
