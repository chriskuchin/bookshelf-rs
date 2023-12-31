import { defineStore } from "pinia";
import { useFiltersStore } from "../filters";

export const useBooksStore = defineStore("books", {
	state: () => ({
		books: [],
		authors: [],
		series: [],
	}),
	actions: {
		async getAuthors() {
			const url = "/api/v1/books/authors";

			const res = await fetch(url);
			this.authors = await res.json();
		},
		async getSeries() {
			const url = "/api/v1/books/series";
			const res = await fetch(url);

			if (res.ok) {
				this.series = await res.json();
			}
		},
		async getBooks(page, size, sort) {
			let url = `/api/v1/books?limit=${size}&offset=${
				size * page
			}&sort=${sort}`;

			const filters = useFiltersStore();
			if (filters.urlFilters !== "") url += `&${filters.urlFilters}`;

			const books = await (await fetch(url)).json();

			if (page === 0) {
				this.books = [];
			}

			for (const book of books) {
				this.books.push(book);
			}
		},
		async deleteBook(bookId) {
			const url = `/api/v1/books/${bookId}`;

			const res = await fetch(url, {
				method: "DELETE",
			});
		},
		async createBook(book) {
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
		async uploadBookFiles(id, files) {
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
