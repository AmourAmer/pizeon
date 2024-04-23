import { createMemoryHistory, createRouter } from "vue-router";

import Menu from "./components/Menu.vue";
import Meals from "./components/Meals.vue";
import Kitchen from "./components/Kitchen.vue";

const routes = [
  { path: "/menu/:type", name: "menu", component: Menu, props: true },
  { path: "/meals", component: Meals },
  { path: "/kitchen", component: Kitchen },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
