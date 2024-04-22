<script setup lang="ts">
import { ref } from "vue";
const props = defineProps(['data'])

const name = ref("");

</script>

<template>
  <div>
    <h2 style="font-size: 72pt">{{ props.data.heading }}</h2>
    <p>{{ props.data.body }}</p>
    <s style="font-size: 36pt">{{ props.data.date.getMonth() }} - {{ props.data.date.getDate() }}</s>
  </div>
</template>
