import { createMemoryHistory, createRouter } from "vue-router";

import Repo from "./components/Repo.vue";
import Meals from "./components/Meals.vue";
import Kitchen from "./components/Kitchen.vue";

const routes = [
  // TODO should redirect / to /meals, just dunno how to do that; or show a greeting page?
  { path: "/", component: Meals },
  { path: "/repo/:type", name: "bill", component: Repo, props: true },
  { path: "/meals", component: Meals },
  { path: "/kitchen", component: Kitchen },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
