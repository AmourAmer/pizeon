<script setup lang="ts">
import { ref } from "vue";
const meals = ref(["1", "2"]);
import Meal from "./Meal.vue";
</script>

<template>
  <div>
    <Meal v-for="(id, i) in meals" :key="i" :id="id" />
  </div>
</template>
