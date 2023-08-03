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
    uploadBookFiles: async function (id, files) {
      console.log(id, files)
      // if (files.length === 0) {
      //   // No files selected
      //   return;
      // }

      // let url = `/api/v1/books/${id}/files/`
      // // Append the files to the FormData object
      // for (let i = 0; i < files.length; i++) {
      //   // Create a new FormData object
      //   const formData = new FormData();
      //   formData.append('file', files[i]);
      // }

      // // Make a POST request to the server with the FormData object
      // let res = await fetch('your-upload-url', {
      //   method: 'POST',
      //   body: formData
      // })

      // if (!res.ok) {
      //   // error
      // } else {
      //   // success
      // }
    }
  }
})
