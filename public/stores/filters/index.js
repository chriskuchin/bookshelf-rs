import { defineStore } from "pinia";

export const useFiltersStore = defineStore("filters", {
	state: () => {
		return {
			author: "",
			series: "",
			title: "",
		};
	},
	getters: {
		getFilters: (state) => {
			const filters = new URLSearchParams();

			if (state.author !== "") {
				filters.append("author", state.author);
			}

			if (state.title !== "") {
				filters.append("title", state.title);
			}

			if (state.series !== "") {
				filters.append("series", state.series);
			}

			return filters.toString();
		},
		isSelectedAuthor: (state) => (author) => author === state.author,
		isSelectedSeries: (state) => (series) => series === state.series,
	},
	actions: {
		setAuthorFilter(author) {
			this.series = "";
			this.title = "";

			this.author = author;
		},
		setTitleFilter(title) {
			this.series = "";
			this.author = "";

			this.title = title;
		},
		setSeriesFilter(series) {
			this.author = "";
			this.title = "";

			this.series = series;
		},
	},
});
