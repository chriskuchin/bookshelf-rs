import { defineStore } from "pinia";

export const useBooksStore = defineStore("books", {
	state: () => {
		return {};
	},
	actions: {
		createBook: async (book) => {
			const url = "/api/v1/books";

			const res = await fetch(url, {
				method: "POST",
				body: JSON.stringify(book),
				headers: {
					"Content-Type": "application/json",
				},
			});

			if (!res.ok) {
				return;
			}

			return await res.json();
		},
		uploadBookFiles: async (id, files) => {
			if (files.length === 0) {
				// No files selected
				return;
			}

			const url = `/api/v1/books/${id}/files`;
			const formData = new FormData();

			for (const k of Object.keys(files)) {
				formData.append(k, files[k], files[k].name);
			}

			// Make a POST request to the server with the FormData object
			const res = await fetch(url, {
				method: "POST",
				body: formData,
			});

			if (!res.ok) {
				// error
			} else {
				// success
			}
		},
	},
});
