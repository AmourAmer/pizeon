<script setup lang="ts">
// TODO: fix type annotation
import { onMounted } from "vue";
import { stringMap } from "@utils/type";
import QRCode from "qrcode";
const props = defineProps<{
  datum: stringMap;
}>();

onMounted(() => {
  var opts = {
    errorCorrectionLevel: "H",
    type: "image/jpeg",
    quality: 0.3,
    margin: 1,
    color: {
      dark: "#010599FF",
      light: "#FFBF60FF",
    },
  };

  QRCode.toDataURL(
    props.datum?.body || "no link found",
    opts,
    function (err, url) {
      if (err) throw err;

      var img = document.getElementById("image");
      console.log(img);
      img.src = url;
    },
  );
});
</script>
<template>
  <div>
    <a :href="datum?.body">{{ datum?.body }}</a>
    <!-- FIXME: unique id -->
    <img id="image" />
  </div>
</template>
