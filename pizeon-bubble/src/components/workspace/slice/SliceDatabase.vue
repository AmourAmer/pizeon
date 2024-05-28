<script setup lang="ts">
import { Ref, ref, computed, watch } from "vue";
import { useTextareaAutosize, toRefs } from "@vueuse/core";
import { stringMap } from "@utils/type";
import { useUpdateType } from "@utils/slice";
import { db } from "./database";

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

const input_disabled = ref(false); // It's hard to give a name if use localstrorage
const disable_input_on_blur = ref(true);
// FIXME:!!!!!!! add btns to toggle auto disable and enable again
watch(textarea, (textarea) => {
  if (!textarea) return;
  textarea.addEventListener(
    "blur",
    () => (input_disabled.value = disable_input_on_blur.value),
  );
});
const toggleDisableInputOnBlur = () => {
  disable_input_on_blur.value = input_disabled.value =
    !disable_input_on_blur.value;
};
</script>

<template>
  <div class="flex">
    <div class="flex-1">
      <input
        type="checkbox"
        :id="datum.id + 'first_line_as_fieldsname'"
        v-model="datum.first_line_as_fieldsname"
      />
      <label :for="datum.id + 'first_line_as_fieldsname'" class="select-none"
        >Use first line as names of fields</label
      >
      <!-- FIXME: how does https://vueuse.org/core/useTextareaAutosize/ impl this? -->
      <textarea
        ref="textarea"
        class="resize-none textarea"
        v-model="datum.body"
        :placeholder="placeholder"
        :rows="5"
        :disabled="input_disabled"
      />
      <!-- TODO: indicate current status? -->
      <button @click="toggleDisableInputOnBlur">Toggle disable on blur</button>
      <button @click="input_disabled = false">Temporarily Enable input</button>
      <!-- TODO: vertical split on small devices -->
    </div>
    <div class="divider divider-horizontal" />
    <div class="flex-1">
      These names will substitute corresponding text in your notice, for
      example:
      <!-- TODO: e.g. -->
      {{ db(datum) }}
      <!-- TODO: toggle empty or follow -->
    </div>
  </div>
</template>
