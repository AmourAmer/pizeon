<script setup lang="ts">
import { Ref, computed } from "vue";
import { useTextareaAutosize } from "@vueuse/core";
import { stringMap } from "@utils/type";
import { useUpdateType, useBindDatum } from "src/utils/slice";

const { textarea, input } = useTextareaAutosize({ styleProp: "minHeight" });

const props = defineProps<{
  servers: string[];
  rValidator: (type: string, datum: Ref<stringMap>) => boolean;
}>();
const datum: Ref<stringMap> = defineModel("datum", { default: {} });
useBindDatum(datum, { body: input });

useUpdateType(datum, { body: input }, props.rValidator);

const placeholder = computed(() => {
  const msg = (dest: string) =>
    "What notice do you want to send on " + dest + "?";
  switch (props.servers.length) {
    case 0:
      return "Please choose a server to send notice to";
    case 1:
      return msg(props.servers[0]);
    case 2:
      return msg(props.servers[0] + " and " + props.servers[1]);
    default:
      return msg(
        props.servers.slice(0, -1).join(", ") +
          ", and " +
          props.servers.slice(-1),
      );
  }
});
</script>

<template>
  <div v-show="!datum.deleted">
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
