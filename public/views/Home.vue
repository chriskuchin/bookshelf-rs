<template>
  <div class="section" style="padding-top: 15px;">
    <div class="box" v-for="book in books" :key="book.uuid">
      <h1 class="title">{{ book.title }}</h1>
      <h2 class="subtitle">{{ book.author }}</h2>
      <fl :files="book.files" :bookID="book.id" />
    </div>
    <div id="sentinel" ref="sentinel" style="height: 10px;"></div>
    <div class="fixed-bottom">
      <a class="button is-primary is-large fab" @click="toggleCreateModal">
        <icon icon="fa-solid fa-plus"></icon>
      </a>
    </div>

    <div :class="['modal', { 'is-active': createModalActive }]">
      <div class="modal-background" @click="toggleCreateModal"></div>
      <div class="modal-content">
        <div class="box">
          <abf @submit="submitCreateBookModal" @cancel="toggleCreateModal" />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import AddBookForm from '../components/AddBookForm.vue'
import FileList from '../components/FileList.vue'
import { mapActions } from 'pinia';
import { useBooksStore } from '../stores/books';

export default {
  components: {
    "abf": AddBookForm,
    "fl": FileList,
  },
  mounted: function () {
    const options = {
      root: null,
      rootMargin: '5px',
      threshold: 0.5,
    };

    const observer = new IntersectionObserver(this.handleIntersection, options);
    observer.observe(this.$refs.sentinel);
  },
  data: function () {
    return {
      books: [],
      sort: {
        key: "title",
        options: [
          "title",
          "author"
        ]
      },
      filter: {
        author: "",
        series: "",
        title: "",
      },
      size: 10,
      page: 0,
      createModalActive: false,
    }
  },
  methods: {
    ...mapActions(useBooksStore, ['createBook', 'uploadBookFiles']),
    handleIntersection: function (entries) {
      if (entries[0].isIntersecting && this.books.length % this.size == 0) {
        this.listBooks(this.page, this.size)
        this.page++
      }
    },
    async listBooks(page, size) {
      let url = `/api/v1/books?limit=${size}&offset=${size * page}&sort=${this.sort.key}`

      if (this.filter.author != "") {
        url += `&author=${encodeURI(this.filter.author)}`
      } else if (this.filter.series != "") {
        url += `&series=${encodeURI(this.filter.series)}`
      } else if (this.filter.title != "") {
        url += `&title=${encodeURI(this.filter.title)}`
      }

      let res = await fetch(url)
      let books = await res.json()

      books.forEach(book => {
        this.books.push(book)
      })
    },
    async submitCreateBookModal(book) {
      this.createBook(book)
      this.uploadBookFiles(book.id, book.files)
      this.toggleCreateModal()
    },
    toggleCreateModal: function () {
      this.createModalActive = !this.createModalActive
    },
  }
}
</script>