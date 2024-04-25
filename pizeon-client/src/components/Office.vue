<script setup lang="ts">
import { toRef } from "vue";
const submitForm = function () {
  // TODO don't forget timestamp and signature
  formData.value = { stamp: "", board: "", body: "", heading: "" };
};
const formData = toRef({ stamp: "", board: "", body: "", heading: "" });
</script>

<template>
  <form @submit.prevent="submitForm">
    <!-- TODO prevent keyStroke -->
    <label for="heading">Heading:</label>
    <input type="heading" id="heading" v-model="formData.heading" />
    <textarea id="body" v-model="formData.body"></textarea>
    <label for="stamp">Stamp:</label>
    <!-- TODO should choose from a list -->
    <input type="text" id="stamp" v-model="formData.stamp" />
    <label for="board">Show at:</label>
    <input type="board" id="board" v-model="formData.board" />
    <button type="submit">Submit</button>
  </form>
</template>
