<script setup lang="ts">
import { onKeyStroke } from "@vueuse/core";
import router from "./router";
import { useI18n } from "vue-i18n";

const { t, locale } = useI18n({ useScope: "global" });

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
// document.body.style.setProperty("zoom", "200%"); // TODO: export as user configurable, or maybe can't use this as tailwind is so powerful
</script>

<template>
  <!-- FIXME: don't know if it's appropriate to put theme here. Also, how to change? -->
  <div>
    <span
      class="sm:bg-sky-100 md:bg-lime-200 lg:bg-orange-200 xl:text-red-700 2xl:text-indigo-800"
    >
      {{ t("message.hello") }}
    </span>
    <nav>
      <RouterLink v-for="(dest, i) in destinations" :to="dest[0]">
        {{ dest[1] }}({{ i + 1 }})
      </RouterLink>
    </nav>
    <div class="flex justify-center">
      <div class="container">
        <router-view v-slot="{ Component }">
          <transition>
            <keep-alive>
              <component :is="Component" />
            </keep-alive>
          </transition>
        </router-view>
      </div>
    </div>
  </div>
</template>
