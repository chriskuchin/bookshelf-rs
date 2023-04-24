<template>
  <div class="section">
    <div class="box" v-for="(book, index) in books" :key="book.uuid">
      <h1 class="title">{{ book.title }}</h1>
      <h2 class="subtitle">{{ book.author }}</h2>
      {{ book }}
      <div class="tags">
        <span class="tag" v-for="(file, index) in book.files" :key="file.path">
          {{ file.type }}
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
    }
  }
}
</script>