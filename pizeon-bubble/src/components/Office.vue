<script setup lang="ts">
import { toRef, Ref, computed, ComputedRef } from "vue";
import Slice from "./Slice.vue";
interface Form {
  server: string;
  body: string;
  signature?: string;
}

// TODO template, cache, sendForm
const submitForm = function () {
  // TODO don't forget timestamp and signature
  formData.value = [];
};
const slices: Ref<string[]> = toRef(["server", "heading", "body"]);
const formData: Ref<(string[] | string)[]> = toRef([]);
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
