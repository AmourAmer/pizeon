<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { Ref } from "vue";
import { useLocalStorage, computedAsync } from "@vueuse/core";

const whoami: Ref<string> = computedAsync(() => invoke("whoami", {}), "sin");

const expert_mode = useLocalStorage("expert_mode", false);
const no_more_confirm = useLocalStorage("no_more_confirm", false);
const toggleExpertMode = () => {
  if (expert_mode.value) return (expert_mode.value = false);
  if (no_more_confirm.value) return (expert_mode.value = true);
  if (confirmExpertMode()) expert_mode.value = true;
};
const confirmExpertMode = () => true; // FIXME:

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
    <!-- I just wanted to declare a no_more_confirm, but ts drove me all the way here -->
    <!-- TODO: use a modal to pop up confirm expert_mode -->
    <button @click="toggleExpertMode">Toggle Expert Mode</button>
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
