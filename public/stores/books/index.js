import { defineStore } from "pinia";


export const useBooksStore = defineStore("books", {
  state: () => {
    return {
    }
  },
  actions: {
    createBook: async function (book) {
      let url = `/api/v1/books`

      let res = await fetch(url, {
        method: "POST",
        body: JSON.stringify(book),
        headers: {
          "Content-Type": "application/json",
        }
      })

      if (!res.ok) {
        return
      }

      return await res.json()
    },
    uploadBookFile: function (id, ext, file) {
      let url = `/api/v1/books/${id}/files/${ext}`
    }
  }
})