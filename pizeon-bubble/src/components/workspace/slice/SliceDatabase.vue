<script setup lang="ts">
import { Ref, ref, computed, watch } from "vue";
import { useTextareaAutosize, toRefs } from "@vueuse/core";
import { stringMap } from "@utils/type";
import { useUpdateType } from "@utils/slice";

const props = defineProps<{
  validator: (type: string, datum: Ref<stringMap>) => boolean;
}>();
const datum: Ref<stringMap> = defineModel("datum", { default: {} });

useUpdateType(datum, ["body"], props.validator);

const textarea: Ref<HTMLTextAreaElement | undefined> = ref();
useTextareaAutosize({
  element: textarea,
  input: toRefs(datum)["body"],
  styleProp: "minHeight",
});

const placeholder = computed(() => "paste table containing needed info here");
// FIXME:!!!!!!! add btns to toggle auto disable and enable again
watch(textarea, (textarea) => {
  if (!textarea) return;
  textarea.addEventListener("blur", () => (textarea.disabled = true));
});
</script>

<template>
  <div>
    <!-- FIXME: how does https://vueuse.org/core/useTextareaAutosize/ impl this? -->
    <textarea
      ref="textarea"
      class="resize-none textarea"
      v-model="datum.body"
      :placeholder="placeholder"
      :rows="5"
    />
    <!-- TODO: toggle empty or follow -->
  </div>
</template>
