import { createMemoryHistory, createRouter } from "vue-router";

import Menu from "./components/Menu.vue";
import Meal from "./components/Meal.vue";
import Kitchen from "./components/Kitchen.vue";
import Fridge from "./components/Menu.vue";
import Unwelcomed from "./components/Menu.vue";
import Junk from "./components/Menu.vue";

const routes = [
  { path: "/menu", component: Menu },
  { path: "/meal", component: Meal },
  { path: "/kitchen", component: Kitchen },
  { path: "/fridge", component: Fridge },
  { path: "/unwelcomed", component: Unwelcomed },
  { path: "/junk", component: Junk },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
