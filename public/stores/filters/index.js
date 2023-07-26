import { defineStore } from "pinia";


export const useFiltersStore = defineStore("filters", {
  state: () => {
    return {
      author: "",
      series: "",
      title: "",
    }
  },
  actions: {
    setAuthorFilter(author) {
      this.series = ""
      this.title = ""

      this.author = author
    },
    setTitleFilter(title) {
      this.series = ""
      this.author = ""

      this.title = title
    },
    setSeriesFilter(series) {
      this.author = ""
      this.title = ""

      this.series = series
    }
  }
})