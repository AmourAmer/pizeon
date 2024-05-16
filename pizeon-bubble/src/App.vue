<script setup lang="ts">
import { onKeyStroke } from "@vueuse/core";
import router from "./router";

const destinations = [
  [{ name: "bill", params: { type: "Fresh" } }, "Fresh Ingredients"],
  ["/meals", "Go to Meals"],
  ["/kitchen", "Go to Kitchen"],
  [{ name: "bill", params: { type: "Fridge" } }, "Open the Fridge"],
  [{ name: "bill", params: { type: "Blocked" } }, "Check outdoors"],
  [{ name: "bill", params: { type: "Junk" } }, "Examine trash can"],
  ["/office", "Go to your Office"],
];

let prefix = "";
// FIXME: to create key-binding like gg, maybe with pinia plugin?
// FIXME: use global table to avoid multi-bind
function normal_mode(
  keys: string | (string | [string, string[]])[],
  fn: (e: KeyboardEvent) => void,
) {
  if (typeof keys == "string") keys = [keys];
  for (let i = 0; i < keys.length; i++) {
    const key = keys[i];
    if (typeof key == "string") keys[i] = ["", [key]];
  }
  for (let i = 0; i < keys.length; i++) {
    const key = keys[i];
    onKeyStroke(key[1], (e) => {
      if (
        e.target &&
        "tagName" in e.target &&
        (e.target.tagName === "INPUT" || e.target.tagName === "TEXTAREA")
      )
        return;

      if (prefix === key[0]) {
        fn(e);
        prefix = "";
      }
      e.preventDefault();
    });
  }
}

for (let i = 0; i < destinations.length; i++) {
  normal_mode([(i + 1).toString()], () => router.push(destinations[i][0]));
}
onKeyStroke("Escape", () => {
  // This is so silly! The type definition of activeElement should be more precise
  (document.activeElement as HTMLElement)?.blur();
});
</script>

<template>
  <div class="container">
    <nav>
      <RouterLink v-for="(dest, i) in destinations" :to="dest[0]">
        {{ dest[1] }}({{ i + 1 }})
      </RouterLink>
    </nav>
    <router-view v-slot="{ Component }">
      <transition>
        <component :is="Component" />
      </transition>
    </router-view>
  </div>
</template>
