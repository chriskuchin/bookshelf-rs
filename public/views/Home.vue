<template>
  <div class="section">
    <div class="box" v-for="(book, index) in books" :key="book.uuid">
      <h1 class="title">{{ book.title }}</h1>
      <h2 class="subtitle">{{ book.author }}</h2>
      <div class="tags">
        <span
          :class="['tag', 'is-rounded', { 'is-primary': getTagName(file.type) == 'mobi' }, { 'is-info': getTagName(file.type) == 'pdf' }, { 'is-warning': getTagName(file.type) == 'epub' }]"
          v-for="(file, index) in book.files" :key="file.path">
          {{ getTagName(file.type) }}
        </span>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  mounted: function () {
    this.listBooks()
  },
  data: function () {
    return {
      books: [],
    }
  },
  methods: {
    async listBooks() {
      let url = "/api/v1/books"
      let res = await fetch(url)

      this.books = await res.json()
    },
    getTagName: function (mime) {
      switch (mime) {
        case "application/x-mobipocket-ebook":
          return "mobi"
        case "application/pdf":
          return "pdf"
        case "application/epub+zip":
          return "epub"
        default:
          return mime
      }
    }
  }
}
</script>