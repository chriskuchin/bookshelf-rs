import { createRouter, createWebHistory } from "vue-router";

import HomePage from "../views/Home.vue";
import AuthorsPage from "../views/Authors.vue";
import SeriesPage from "../views/Series.vue";

const routes = [
	{ path: "/", component: HomePage },
	{ path: "/authors", component: AuthorsPage },
	{ path: "/series", component: SeriesPage },
];

const router = createRouter({
	history: createWebHistory(),
	routes,
});

export default router;
