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

const placeholder = computed(() => {
  const msg = (dest: string) =>
    "What notice do you want to send to " + dest + "?";
  switch (props.destinations.length) {
    case 0:
      return "Please choose a server or email address to send notice to";
    case 1:
      if (props.destinations[0].length > 0) return msg(props.destinations[0]);
      else return "Please specify an email address to send notice to";
    case 2:
      return msg(props.destinations[0] + " and " + props.destinations[1]);
    default:
      return msg(
        props.destinations.slice(0, -1).join(", ") +
          ", and " +
          props.destinations.slice(-1),
      );
  }
});
</script>

<template>
  <div>
    <!-- FIXME: how does https://vueuse.org/core/useTextareaAutosize/ impl this? -->
    <textarea
      ref="textarea"
      class="resize-none"
      v-model="input"
      :placeholder="placeholder"
      :rows="3"
    />
  </div>
</template>
