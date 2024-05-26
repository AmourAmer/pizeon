<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref } from "vue";
import { useStorage, computedAsync } from "@vueuse/core";

const whoami: Ref<string> = computedAsync(() => invoke("whoami", {}), "sin");

const expert_mode = useStorage("expert_mode", false);
const servers = computedAsync(
  () =>
    (
      invoke("get_servers", {
        role: whoami.value,
      }) as Promise<string[]>
    ).then((res: string[]) => res.map((r) => r.split(".").pop() as string)),
  [whoami.value],
);
// Q: if use splice(-1)[0] instead, won't infer undefined. Why? pop doesn't know split always non-empty?
</script>

<template>
  <div>
    <p v-if="expert_mode">
      Expert Mode is currently unavailable, wait for months and update your app
      to see if I'll've implemented that
    </p>
    <!-- TODO: should I use h1? Should learn about this. -->
    <h1>Who are you</h1>
    {{ whoami }}
    <h1>Who you trust</h1>
    <h1>Where you deliver</h1>
    [{{ servers }}]
    <h1>What you do</h1>
  </div>
</template>
