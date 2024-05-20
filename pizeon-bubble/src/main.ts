import { createApp } from "vue";
import { createI18n } from "vue-i18n";
import router from "./router";
import "./styles.css";
import App from "./App.vue";

const i18n = createI18n({
  legacy: false,
  locale: "zh-CN",
  fallbackLocale: "en",
  messages: {
    en: {
      message: {
        hello: "hello world",
      },
    },
    "zh-CN": {
      message: {
        hello: "Hello世界",
      },
    },
  },
});

createApp(App).use(router).use(i18n).mount("#app");
