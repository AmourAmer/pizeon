<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, Ref, computed } from "vue";
import Event from "./workspace/Event.vue";
import { stringMap } from "../utils/type";

// FIXME: refactor
const template = ref("event");
// TODO: should use some generic type, maybe helpful: https://vuejs.org/guide/typescript/composition-api.html#typing-component-template-refs
const notePage = ref<InstanceType<typeof Event> | null>(null);
// TODO: template, cache, sendForm
const submitForm = () => {
  // TODO: don't forget signature
  let bundle: stringMap = { raw: notePage.value?.finalize() } || {};
  bundle.servers = servers.value;
  bundle.signature = signature.value;
  bundle.template = template.value;
  invoke("send_notice", {
    servers: servers.value,
    body: JSON.stringify(bundle),
    signatures: [signature.value].map((s) => s),
  });
  // TODO: ~~send back a notice containing server respone~~
};
const servers: Ref<string[]> = ref(["self"]);
const signature: Ref<string> = ref("self");

const templateComponent = computed(() => {
  switch (template.value) {
    case "event":
      return Event;
    default:
      return Event;
  }
});
</script>

<template>
  <div>
    <div>
      Send to:
      <select multiple v-model="servers">
        <option :value="'self'">self</option>
        <option :value="'test 1'">test 1</option>
        <option :value="'test 3'">test 3</option>
      </select>
    </div>
    <div>
      Sent by:
      <select v-model="signature">
        <option :value="'self'">self</option>
        <option :value="'test 2'">test 2</option>
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
    <component :is="templateComponent" ref="notePage" :servers="servers" />
    <!-- FIXME: export and copy on submitting -->
    <button @click="submitForm">Publish</button>
    <footer>
      If parsing isn't satisfying or anything, plz
      mailto:Amour&lt;pizeon@tuta.io&gt;
    </footer>
    <!-- TODO: click to open mailto: link and use ctx as body -->
  </div>
</template>
