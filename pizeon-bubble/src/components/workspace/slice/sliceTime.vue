<script setup lang="ts">
import { Ref, computed } from "vue";
import { useTextareaAutosize } from "@vueuse/core";
// FIXME: there must be some way to use absolute path!
import { stringMap } from "@utils/type";
import { useUpdateType, useUpdateDatum, useInitCheck } from "src/utils/slice";

const { textarea, input } = useTextareaAutosize({ styleProp: "minHeight" });

const props = defineProps<{
  servers: string[];
  rValidator: (type: string, datum: stringMap) => false | string;
}>();
const datum: Ref<stringMap> = defineModel("datum", { default: {} });
if (datum.value.body) {
  input.value = datum.value.body;
}
// datum.value.body = input; // with this line, the update dependency bug triggers for only sliceTime. Before this commit, it triggers for both this and sliceTextarea. Unfortunately, I didn't commit all files in the buggy state. Never expect such painful to lose track of a bug
useUpdateDatum(datum, { body: input });
useInitCheck(datum, { body: input });

const warning = useUpdateType(input, datum, props.rValidator);

const placeholder = computed(() => "When does it take place?");
</script>

<template>
  <div v-show="!datum.deleted">
    TIME:
    <textarea
      ref="textarea"
      class="resize-none"
      v-model="input"
      :placeholder="placeholder"
      :rows="3"
    />
    {{ warning }}
  </div>
</template>

<style scoped>
textarea {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

textarea::-webkit-scrollbar {
  display: none;
}

.resize-none {
  resize: none;
}
</style>
