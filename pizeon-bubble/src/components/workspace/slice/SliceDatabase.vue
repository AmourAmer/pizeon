<script setup lang="ts">
import { Ref, ref, computed, watch } from "vue";
import { useTextareaAutosize, toRefs } from "@vueuse/core";
import { stringMap } from "@utils/type";
import { useUpdateType } from "@utils/slice";
import { db, process_db, parseDB } from "./database";

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

const fieldsName = computed(() => process_db(datum.value).fields);
watch(
  () => parseDB(datum.value)[0].length,
  (length) =>
    (datum.value.fieldsname = Array.from({ length }, (_, i) => "v" + i)),
);
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
      <div class="overflow-x-auto">
        <table class="table table-xs table-pin-rows table-pin-cols">
          <thead>
            <th></th>
            <td
              v-for="(f, i) in fieldsName"
              :class="i == datum.key ? 'text-red-500' : ''"
              @click="datum.key = i"
            >
              <span v-if="datum.first_line_as_fieldsname">{{ f }}</span>
              <input type="text" v-else v-model="datum.fieldsname[i]" />
            </td>
          </thead>
          <tbody>
            <tr v-for="(l, j) in process_db(datum).mapper" class="hover">
              <th>{{ j }}</th>
              <td
                v-for="(v, i) in l"
                :class="i == datum.key ? 'text-red-500' : ''"
              >
                {{ v }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      {{ process_db(datum) }}
      These names will substitute corresponding text in your notice, for
      example:
      <!-- TODO: e.g. -->
      {{ db(datum) }}
      <!-- TODO: toggle empty or follow -->
    </div>
  </div>
</template>
