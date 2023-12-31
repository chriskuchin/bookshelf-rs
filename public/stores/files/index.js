import { defineStore } from "pinia";

export const useFilesStore = defineStore("files", {
	state: () => {
		return {};
	},
	actions: {
		deleteFile: async (bookId, fileType) => {
			const url = `/api/v1/books/${bookId}/files/${fileType}`;

			const res = await fetch(url, {
				method: "DELETE",
			});
		},
	},
});
