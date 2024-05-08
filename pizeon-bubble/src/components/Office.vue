<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, Ref, computed, ComputedRef } from "vue";
import { useStorage } from "@vueuse/core";
import Slice from "./workspace/Slice.vue";

interface stringMap {
  [key: string]: string | string[];
}

const template = ref("classic");
// TODO: template, cache, sendForm
const submitForm = function () {
  // TODO: don't forget timestamp and signature
  // FIXME: don't forget notice type
  let bundle: stringMap = {};
  for (let i = 1; i < formData.value.length; i++) {
    bundle[slices.value[i]] = formData.value[i];
  }
  bundle.template = template.value;
  invoke("send_notice", {
    servers: formData.value[0],
    body: JSON.stringify(bundle),
    signatures: (formData.value[formData.value.length - 1] as string[]).map(
      (s) => s,
    ),
  });
  // TODO: ~~send back a notice containing server respone~~
  formData.value = initFormData();
};
const templateTo = () => {
  let slices: string[] = [];
  switch (template.value) {
    case "hell":
      slices.push();
      break;
    default:
      slices.push("heading", "raw");
  }
  return ["server"].concat(slices).concat("signature");
};
const initFormData = function () {
  return templateTo().map((s) =>
    s === "server" || s === "signature" ? ["self"] : "",
  );
};
const slices: Ref<string[]> = computed(templateTo);
const formData: Ref<(string[] | string)[]> = useStorage(
  template.value,
  initFormData(),
); // TODO: multi-account?!
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
    <!-- FIXME: export and copy on submitting -->
    <button type="submit">Submit</button>
  </form>
</template>
