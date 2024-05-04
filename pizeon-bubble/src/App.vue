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

for (let i = 0; i < destinations.length; i++) {
  onKeyStroke([(i + 1).toString()], (e) => {
    if (
      e.target &&
      "tagName" in e.target &&
      (e.target?.tagName === "INPUT" || e.target?.tagName === "TEXTAREA")
    )
      return;
    router.push(destinations[i][0]);
  });
}
</script>

<template>
  <div class="container">
    <nav>
      <RouterLink v-for="(dest, i) in destinations" :key="i" :to="dest[0]">
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
