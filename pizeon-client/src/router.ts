import { createMemoryHistory, createRouter } from "vue-router";

import Repo from "./components/Repo.vue";
import Meals from "./components/Meals.vue";
import Kitchen from "./components/Kitchen.vue";

const routes = [
  { path: "/repo/:type", name: "repo", component: Repo, props: true },
  { path: "/meals", component: Meals },
  { path: "/kitchen", component: Kitchen },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
