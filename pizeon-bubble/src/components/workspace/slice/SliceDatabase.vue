<script setup lang="ts">
import { Ref, ref, computed } from "vue";
import { useTextareaAutosize, toRefs } from "@vueuse/core";
import { stringMap } from "@utils/type";
import { useUpdateType } from "@utils/slice";

const props = defineProps<{
  validator: (type: string, datum: Ref<stringMap>) => boolean;
}>();
const datum: Ref<stringMap> = defineModel("datum", { default: {} });

useUpdateType(datum, ["body"], props.validator);

const textarea = ref();
useTextareaAutosize({
  element: textarea,
  input: toRefs(datum)["body"],
  styleProp: "minHeight",
});

const placeholder = computed(() => "paste table containing needed info here");
</script>

<template>
  <div>
    <!-- FIXME: how does https://vueuse.org/core/useTextareaAutosize/ impl this? -->
    <textarea
      ref="textarea"
      class="resize-none"
      v-model="datum.body"
      :placeholder="placeholder"
      :rows="5"
    />
  </div>
</template>
