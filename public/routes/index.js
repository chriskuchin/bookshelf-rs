import { createRouter, createWebHistory } from "vue-router";

import HomePage from "../views/Home.vue";
import ReadPage from "../views/Read.vue";

const routes = [
	{ path: "/", component: HomePage },
	{ path: "/read", component: ReadPage },
];

const router = createRouter({
	history: createWebHistory(),
	routes,
});

export default router;
