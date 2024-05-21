<script setup lang="ts">
// TODO: fix type annotation
import { stringMap } from "@utils/type";
import QRCode from "qrcode";
import { QRCodeToDataURLOptions } from "qrcode";
import { computedAsync } from "@vueuse/core";
const props = defineProps<{
  datum: stringMap;
}>();

const opts = {
  errorCorrectionLevel: "L",
  type: "image/svg",
  margin: 1,
  color: {
    dark: "#000",
    light: "#fff6",
  },
};

const src = computedAsync(
  () =>
    QRCode.toDataURL(props.datum?.body || "/", opts as QRCodeToDataURLOptions),
  "",
);
</script>
<template>
  <div>
    <!-- TODO: toggle -->
    <a :href="datum?.body">
      {{ datum?.body || "no url found" }}
    </a>
    <img :src="src" />
  </div>
</template>
