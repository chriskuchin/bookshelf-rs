import { createApp } from "vue";
import App from "./App.vue";

import { createPinia } from "pinia";
import router from "./routes";

import("./css/bookshelf.scss");

/* import the fontawesome core */
import { library } from "@fortawesome/fontawesome-svg-core";

/* import font awesome icon component */
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

/* import specific icons */
import {
	faCheck,
	faEllipsis,
	faEllipsisV,
	faPencil,
	faPlus,
	faUpload,
} from "@fortawesome/free-solid-svg-icons";

if ("serviceWorker" in navigator) {
	window.addEventListener("load", () => {
		navigator.serviceWorker
			.register("/service-worker.js")
			.then((registration) => {
				// console.log("SW registered: ", registration);
			})
			.catch((registrationError) => {
				// console.log("SW registration failed: ", registrationError);
			});
	});
}

library.add(faPlus, faCheck, faPencil, faUpload, faEllipsisV);

const pinia = createPinia();
createApp(App)
	.component("icon", FontAwesomeIcon)
	.use(pinia)
	.use(router)
	.mount("#app");

require("./assets/");
