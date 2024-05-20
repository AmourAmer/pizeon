<script setup lang="ts">
import { Ref, computed } from "vue";
import { useTextareaAutosize } from "@vueuse/core";
// FIXME: there must be some way to use absolute path!
import { stringMap } from "@utils/type";
import { useUpdateType, useReadDatum } from "src/utils/slice";

const { textarea, input } = useTextareaAutosize({ styleProp: "minHeight" });
// TODO: maybe fix-height is just ok
const { textarea: textarea_name, input: input_name } = useTextareaAutosize({
  styleProp: "minHeight",
});

const props = defineProps<{
  servers: string[];
  rValidator: (type: string, datum: Ref<stringMap>) => boolean;
}>();
const datum: Ref<stringMap> = defineModel("datum", { default: {} });
useReadDatum(datum, { name: input_name, description: input });

useUpdateType(
  datum,
  { name: input_name, description: input },
  props.rValidator,
);

const placeholder = computed(() => "Description of this guy");
const placeholder_name = computed(() => "Who's it?");
</script>

<template>
  <div v-show="!datum.deleted">
    <!-- FIXME: name, cv, avatar etc. -->
    Host:
    <!-- TODO: avatar -->
    <p>
      Avatar will be supported later, if anybody has any idea how to impl this,
      plz contact me, thx!
    </p>
    NAME:
    <textarea
      ref="textarea_name"
      class="resize-none"
      v-model="input_name"
      :placeholder="placeholder_name"
      :rows="1"
    />
    DESCRIPTION:
    <textarea
      ref="textarea"
      class="resize-none"
      v-model="input"
      :placeholder="placeholder"
      :rows="3"
    />
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
