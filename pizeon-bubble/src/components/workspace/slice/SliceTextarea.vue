<script setup lang="ts">
import { Ref, computed } from "vue";
import { useTextareaAutosize } from "@vueuse/core";
import { stringMap } from "@utils/type";
import { useUpdateType, useBindDatum } from "src/utils/slice";

const { textarea, input } = useTextareaAutosize({ styleProp: "minHeight" });

const props = defineProps<{
  destinations: string[];
  rValidator: (type: string, datum: Ref<stringMap>) => boolean;
}>();
const datum: Ref<stringMap> = defineModel("datum", { default: {} });
useBindDatum(datum, { body: input });

useUpdateType(datum, { body: input }, props.rValidator);

const { placeholderFn } = await import(`./${datum.value["type"]}.ts`); // TODO: error handling
const placeholder = computed(() => placeholderFn(props.destinations));
</script>

<template>
  <div>
    {{ datum.type.toUpperCase() }}:
    <!-- FIXME: how does https://vueuse.org/core/useTextareaAutosize/ impl this? -->
    <!-- FIXME: Ctrl-Z -->
    <textarea
      ref="textarea"
      class="resize-none"
      v-model="input"
      :placeholder="placeholder"
      :rows="3"
    />
  </div>
</template>
