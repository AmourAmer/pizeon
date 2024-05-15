<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, Ref, computed, toRef } from "vue";
import { useStorage } from "@vueuse/core";
import Event from "./workspace/Event.vue";

interface stringMap {
  [key: string]: string | string[];
}

// FIXME: refactor
const template = ref("event");
// TODO: template, cache, sendForm
const submitForm = function () {
  // TODO: don't forget signature
  // FIXME: don't forget notice template type
  let bundle: stringMap = {};
  // FIXME: of course, since this's been refactored
  for (let i = 1; i < formData.value.length; i++) {
    bundle[slices.value[i]] = formData.value[i];
  }
  bundle.template = template.value;
  invoke("send_notice", {
    servers,
    body: JSON.stringify(bundle),
    signatures: [signature].map((s) => s),
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
// TODO: this is kind of ugly, should polish, also in ./workspace/*
const formData: Ref<(string[] | string)[]> = useStorage(
  template.value,
  initFormData(),
); // TODO: multi-account?!
const servers: Ref<string[]> = toRef(["self"]);
const signature: Ref<string> = toRef("self");

const templateComponent = computed(() => {
  switch (template.value) {
    case "event":
      return Event;
    default:
      return Event;
  }
});
const templateData = toRef([]);
</script>

<template>
  <div>
    <div>
      Send to:
      <select multiple v-model="servers">
        <option :value="'self'">self</option>
        <option :value="'test 1'">test 1</option>
      </select>
    </div>
    <div>
      Sent by:
      <select v-model="signature">
        <option :value="'self'">self</option>
        <option :value="'test 1'">test 1</option>
      </select>
    </div>
    <div>
      Template:
      <select v-model="template">
        <!-- <option value="classic">classic</option> -->
        <option :value="'event'">event</option>
      </select>
      {{ template }}
    </div>
    <component
      :is="templateComponent"
      v-model="templateData"
      :servers="servers"
    />
    <!-- FIXME: export and copy on submitting -->
    <button @click="submitForm">Publish</button>
    <footer>
      If parsing isn't satisfying or anything, plz
      mailto:Amour&lt;pizeon@tuta.io&gt;
    </footer>
    <!-- TODO: click to open mailto: link and use ctx as body -->
  </div>
</template>
