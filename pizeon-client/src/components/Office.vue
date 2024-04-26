<script setup lang="ts">
import { toRef, Ref } from "vue";
import Slice from "./Slice.vue";
interface Form {
  server: string;
  signature?: string;
}

// TODO template, cache, sendForm
const submitForm = function () {
  // TODO don't forget timestamp and signature
  formData.value = [];
};
const slices: Ref<string[]> = toRef(["body"]);
const formData = toRef([""]);
</script>

<template>
  <form @submit.prevent="submitForm">
    <Slice
      v-for="(slice, i) in slices"
      :key="i"
      v-model="formData[i]"
      :slice="slice"
    />
    <button type="submit">Submit</button>
  </form>
</template>
