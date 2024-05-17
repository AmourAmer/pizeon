<script setup lang="ts">
import { Ref, computed } from "vue";
import { useTextareaAutosize } from "@vueuse/core";
// FIXME: there must be some way to use absolute path!
import { stringMap } from "../../../utils/type";

const { textarea, input } = useTextareaAutosize({ styleProp: "minHeight" });

const props = defineProps<{
  servers: string[];
}>();
const datum: Ref<stringMap> = defineModel("datum", { default: {} });
if (datum.value.body) {
  input.value = datum.value.body;
}
datum.value.body = input;

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
    C{{ datum }}C D{{ input }}D
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
