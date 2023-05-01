<template>
  <div class="section" style="padding-top: 15px;">
    <div class="box" v-for="book in books" :key="book.uuid">
      <h1 class="title">{{ book.title }}</h1>
      <h2 class="subtitle">{{ book.author }}</h2>
      <div class="downloads">
        <a class="tags has-addons" v-for="file in book.files" :key="file.path" :href="getDownloadLink(book, file)"
          download>
          <span v-if="getFormatModifier(file.type) != ''" :class="['tag', 'is-rounded', 'is-dark']">
            {{ getFormatModifier(file.type) }}
          </span>
          <span
            :class="['tag', 'is-rounded', { 'is-primary': getFileFormat(file.type) == 'mobi' }, { 'is-info': getFileFormat(file.type) == 'pdf' }, { 'is-warning': getFileFormat(file.type) == 'epub' }]">
            {{ getFileFormat(file.type) }}
          </span>
        </a>
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
    getFileFormat: function (mime) {
      let format = this.getTagName(mime)
      let formatParts = format.split(".")

      if (formatParts.length == 1) {
        return format
      } else {
        return formatParts[1]
      }
    },
    getFormatModifier: function (mime) {
      let format = this.getTagName(mime)
      let formatParts = format.split(".")

      if (formatParts.length == 1) {
        return ""
      } else {
        return formatParts[0]
      }
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
    },
    getDownloadLink: function (book, file) {
      return "/api/v1/books/" + book.id + "/files/" + this.getTagName(file.type)
    }
  }
}
</script>