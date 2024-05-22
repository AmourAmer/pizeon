<script setup lang="ts">
import { Ref, computed } from "vue";
import { useTextareaAutosize } from "@vueuse/core";
import { stringMap } from "@utils/type";
import { useUpdateType, useBindDatum } from "src/utils/slice";

const { textarea, input } = useTextareaAutosize({ styleProp: "minHeight" });
// TODO: maybe fix-height is just ok
const { textarea: textarea_name, input: input_name } = useTextareaAutosize({
  styleProp: "minHeight",
});
const { textarea: textarea_title, input: input_title } = useTextareaAutosize({
  styleProp: "minHeight",
});

const props = defineProps<{
  rValidator: (type: string, datum: Ref<stringMap>) => boolean;
}>();
const datum: Ref<stringMap> = defineModel("datum", { default: {} });
useBindDatum(datum, {
  name: input_name,
  title: input_title,
  description: input,
});

useUpdateType(datum, { description: input }, props.rValidator);

const placeholder = computed(() => "Description of this guy");
const placeholder_name = computed(() => "Who's it?");
const placeholder_title = computed(() => "Title?");
</script>

<template>
  <div v-show="!datum.deleted">
    <!-- FIXME: name, cv, avatar etc. -->
    Host:
    <!-- TODO: avatar -->
    <p>
      Avatar will be supported later, if anybody has any idea how to impl this,
      plz contact me, thx! <br />
      For now, let's just use O.o
    </p>
    NAME:
    <textarea
      ref="textarea_name"
      class="resize-none"
      v-model="input_name"
      :placeholder="placeholder_name"
      :rows="1"
    />
    <!-- TODO: extract these logic to a helper vue SFC, according to the commit containing this line. Should have done it now, sorry about this. The only potential problem I see is passing datum to grandson, which is, not-that-bad actually. Also remember to add a prop about rows. -->
    TITLE:
    <textarea
      ref="textarea_title"
      class="resize-none"
      v-model="input_title"
      :placeholder="placeholder_title"
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
