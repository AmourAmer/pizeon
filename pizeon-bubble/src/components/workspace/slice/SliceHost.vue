<script setup lang="ts">
import { Ref, ref, computed } from "vue";
import { useTextareaAutosize, toRefs } from "@vueuse/core";
import { stringMap } from "@utils/type";
import { useUpdateType } from "@utils/slice";

const props = defineProps<{
  validator: (type: string, datum: Ref<stringMap>) => boolean;
}>();
const datum: Ref<stringMap> = defineModel("datum", { default: {} });

useUpdateType(datum, ["description"], props.validator);

const textarea_name = ref();
useTextareaAutosize({
  element: textarea_name,
  input: toRefs(datum)["name"],
  styleProp: "minHeight",
});
const textarea_title = ref();
useTextareaAutosize({
  element: textarea_title,
  input: toRefs(datum)["title"],
  styleProp: "minHeight",
});
const textarea = ref();
useTextareaAutosize({
  element: textarea,
  input: toRefs(datum)["description"],
  styleProp: "minHeight",
});

const placeholder_name = computed(() => "Who's it?");
const placeholder_title = computed(() => "Title?");
const placeholder = computed(() => "Description of this guy");
</script>

<template>
  <div>
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
      class="resize-none textarea"
      v-model="datum.name"
      :placeholder="placeholder_name"
      :rows="1"
    />
    <!-- TODO: extract these logic to a helper vue SFC, according to the commit containing this line. Should have done it now, sorry about this. The only potential problem I see is passing datum to grandson, which is, not-that-bad actually. Also remember to add a prop about rows. -->
    TITLE:
    <textarea
      ref="textarea_title"
      class="resize-none textarea"
      v-model="datum.title"
      :placeholder="placeholder_title"
      :rows="1"
    />
    DESCRIPTION:
    <textarea
      ref="textarea"
      class="resize-none textarea"
      v-model="datum.description"
      :placeholder="placeholder"
      :rows="3"
    />
  </div>
</template>
