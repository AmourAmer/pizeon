import { createMemoryHistory, createRouter } from "vue-router";

import Menu from "./components/Menu.vue";
import Meal from "./components/Meal.vue";
import Kitchen from "./components/Kitchen.vue";
import Fridge from "./components/Menu.vue";
import Unwelcomed from "./components/Menu.vue";
import Junk from "./components/Menu.vue";

const routes = [
  { path: "/menu/:type", name: "menu", component: Menu },
  { path: "/meal", component: Meal },
  { path: "/kitchen", component: Kitchen },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
