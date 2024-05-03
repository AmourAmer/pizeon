<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, Ref, computed, ComputedRef } from "vue";
import { useStorage } from "@vueuse/core";
import Slice from "./workspace/Slice.vue";

const template = ref("classic");
// TODO: template, cache, sendForm
const submitForm = function () {
  // TODO: don't forget timestamp and signature
  // FIXME: don't forget notice type
  let bundle = {};
  invoke("send_notice", {
    servers: formData.value[0],
    body: JSON.stringify(bundle),
    // signature: formData.value.slice(-1)
  }),
    // TODO: ~~send back a notice containing server respone~~
    (formData.value = []);
};
const templateTo = () => {
  let slices: string[] = [];
  switch (template.value) {
    case "hell":
      slices.push();
      break;
    default:
      slices.push("heading", "body");
  }
  return ["server"].concat(slices).concat("signature");
};
const slices: Ref<string[]> = computed(templateTo);
const formData: Ref<(string[] | string)[]> = useStorage(template.value, []); // TODO: multi-account?!
const server: ComputedRef<string[]> = computed(
  () => (formData.value[slices.value.indexOf("server")] || []) as string[],
);
</script>

<template>
  <form @submit.prevent="submitForm">
    <Slice
      v-for="(slice, i) in slices"
      :key="i"
      v-model="formData[i]"
      :slice="slice"
      :server="server"
    />
    <button type="submit">Submit</button>
  </form>
</template>
