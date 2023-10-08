<template>
  <div class="section" style="padding-top: 15px;">
    <div class="box" v-for="book in books" :key="book.uuid">
      <div class="has-text-right">
        <div class="dropdown is-hoverable is-right">
          <div class="dropdown-trigger">
            <span class="icon is-clickable" aria-haspopup="true" aria-controls="dropdown-menu">
              <icon icon="fa-solid fa-ellipsis-v"></icon>
            </span>
          </div>
          <div class="dropdown-menu" id="dropdown-menu" role="menu">
            <div class="dropdown-content">
              <a class="dropdown-item" @click="openUploadModal(book.id)">
                <icon icon="fa-solid fa-upload"></icon>
                Upload Files
              </a>
            </div>
          </div>
        </div>
      </div>

      <h1 class="title">
        {{ book.title }}
      </h1>
      <!-- <a @click=""><icon icon="fa-solid fa-pencil"></icon></a> -->
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

    <div :class="['modal', { 'is-active': uploadModal.active }]">
      <div class="modal-background" @click="toggleUploadModal"></div>
      <div class="modal-content">
        <div class="box">
          <uff :bookID="uploadModal.id" @submit="submitUploadModal" @cancel="toggleUploadModal" />
        </div>
      </div>
    </div>

  </div>
</template>

<script>
import AddBookForm from '../components/AddBookForm.vue'
import UploadFileForm from '../components/UploadFileForm.vue'
import FileList from '../components/FileList.vue'
import { mapActions, mapState } from 'pinia';
import { useBooksStore } from '../stores/books';
import { useFiltersStore } from '../stores/filters';

export default {
  components: {
    "abf": AddBookForm,
    "uff": UploadFileForm,
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

    const filterStore = useFiltersStore()
    var that = this
    filterStore.$subscribe((mut, state) => {
      that.page = 0
      that.books = []
      that.listBooks(that.page, that.size)
    })
  },
  data: function () {
    return {
      uploadModal: {
        active: false,
        id: "",
      },
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
  computed: {},
  methods: {
    ...mapState(useFiltersStore, ['getFilters']),
    ...mapActions(useBooksStore, ['createBook', 'uploadBookFiles']),
    handleIntersection: function (entries) {
      if (entries[0].isIntersecting && this.books.length % this.size == 0) {
        this.listBooks(this.page, this.size)
        this.page++
      }
    },
    async listBooks(page, size) {
      let url = `/api/v1/books?limit=${size}&offset=${size * page}&sort=${this.sort.key}`

      url += `&${this.getFilters()}`

      let res = await fetch(url)
      let books = await res.json()

      books.forEach(book => {
        this.books.push(book)
      })
    },
    async submitCreateBookModal(book, files) {
      let id = await this.createBook(book)
      this.uploadBookFiles(id, files)
      this.toggleCreateModal()
    },
    toggleCreateModal: function () {
      this.createModalActive = !this.createModalActive
    },
    async submitUploadModal(id, files) {
      this.uploadBookFiles(id, files)
      this.toggleUploadModal()
      this.uploadModal.id = ""
    },
    toggleUploadModal: function () {
      this.uploadModal.active = !this.uploadModal.active
    },
    openUploadModal: function (bookID) {
      this.uploadModal.id = bookID
      this.uploadModal.active = true
    }
  }
}
</script>