<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, Ref, computed } from "vue";
import { useTextareaAutosize } from "@vueuse/core";
import Event from "./workspace/Event.vue";
import PreviewEvent from "./meal/Event.vue";
import { stringMap } from "@utils/type";
import { splitEmailAddress } from "@utils/email";

// FIXME: refactor
const template = ref("event");
// TODO: should use some generic type, maybe helpful: https://vuejs.org/guide/typescript/composition-api.html#typing-component-template-refs
const draftPage = ref<InstanceType<typeof Event> | null>(null);
// TODO: template, cache, sendForm
const submitForm = () => {
  if (!previewShow.value) {
    // TODO: should have a name
    foo.value =
      "Must open Preview when Publishing, this also helps to prevent publishing by mistake. This behavior will be configurable in the future";
    return;
  }
  // TODO: don't forget signature
  let bundle: stringMap = draftPage.value?.finalize() || {};
  bundle.destinations = destinations.value;
  bundle.signature = [signature.value];
  bundle.template = template.value;
  console.log(bundle);
  invoke("send_notice", {
    destinations: destinations.value,
    body: JSON.stringify(bundle),
    signatures: [signature.value].map((s) => s),
  });
  // TODO: ~~send back a notice containing server respone~~
};
const foo = ref("");
const servers: Ref<string[]> = ref(["self"]);
const destinations = computed(() => {
  switch (receiver.value) {
    case "server":
      return servers.value;
    case "address":
      return [splitEmailAddress(receivers.value).join(";")];
    default:
      return servers.value;
  }
}); // FIXME: server-on/to
const signature: Ref<string> = ref("self");

const templateComponent = computed(() => {
  switch (template.value) {
    case "event":
      return Event;
    default:
      return Event;
  }
});

// TODO: duplicated, seems silly
const previewComponent = computed(() => {
  switch (template.value) {
    case "event":
      return PreviewEvent;
    default:
      return PreviewEvent;
  }
});

const previewData = computed(() => draftPage.value?.preview() || "");

const previewShow = ref(false);
const togglePreview = () => {
  previewShow.value = !previewShow.value;
  foo.value = "";
};

const { textarea, input: receivers } = useTextareaAutosize({
  styleProp: "minHeight",
});
const receiver = ref("server");
const nextReceiver = () => {
  switch (receiver.value) {
    case "address":
      return "server";
    case "server":
      return "address";
    default:
      return "server";
  }
};
const toggleReceiver = () => (receiver.value = nextReceiver());
const nReceiver = computed(() => {
  return "use " + nextReceiver();
});
</script>

<template>
  <div>
    <div>
      Send to:
      <select multiple v-model="servers" v-if="receiver == 'server'">
        <option :value="'self'">self</option>
        <option :value="'test 1'">test 1</option>
        <option :value="'test 3'">test 3</option>
      </select>
      <!-- TODO: this is really ugly textarea -->
      <textarea
        ref="textarea"
        class="resize-none"
        v-model="receivers"
        :rows="1"
        v-else-if="receiver == 'address'"
      />
      <button @click="toggleReceiver">{{ nReceiver }}</button>
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
    </div>
    <component :is="templateComponent" ref="draftPage" :servers="servers" />
    <!-- FIXME: export and copy on submitting -->
    <button @click="togglePreview">Toggle Preview</button>
    <button @click="submitForm">Publish</button>
    {{ foo }}
    <component v-if="previewShow" :is="previewComponent" :data="previewData" />
    <footer>
      If parsing isn't satisfying or anything, plz
      mailto:Amour&lt;pizeon@tuta.io&gt;
    </footer>
    <!-- TODO: click to open mailto: link and use ctx as body -->
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
