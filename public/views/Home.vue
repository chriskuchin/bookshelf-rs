<template>
  <div class="section" style="padding-top: 15px;">
    <div class="box mb-4" v-for="book in books" :key="book.uuid">
      <div class="has-text-right">
        <div class="dropdown is-right" @click="clickMenu">
          <div class="dropdown-trigger">
            <span class="icon" aria-haspopup="true" aria-controls="dropdown-menu">
              <icon icon="fa-solid fa-ellipsis-v"></icon>
            </span>
          </div>
          <div class="dropdown-menu" id="dropdown-menu" role="menu">
            <div class="dropdown-content">
              <a class="dropdown-item" @click="openUploadModal(book.id)">
                <icon icon="fa-solid fa-upload"></icon>
                Upload Files
              </a>
              <a class="dropdown-item" href="#">{{ book.id }}</a>
              <hr class="dropdown-divider" />
              <a class="dropdown-item" @click="deleteBookClick(book.id)">Delete Book</a>
              <hr v-if="book.files.length > 0" class="dropdown-divider" />
              <a v-for="file in book.files" class="dropdown-item" @click="deleteFileClick(book.id, file.type)">
                Delete {{ file.type }}
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
import { useFilesStore } from '../stores/files';

export default {
  components: {
    "abf": AddBookForm,
    "uff": UploadFileForm,
    "fl": FileList,
  },
  created: function () {
    this.$watch(
      () => this.$route.params,
      (toParams, previousParams) => {
        // react to route changes...
        console.log(toParams, previousParams)
      }
    )
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
      that.getBooks(that.page, that.size, that.sort.key)
    })

    if (this.$route.query["author"]) {
      filterStore.setAuthorFilter(this.$route.query["author"])
    }

    if (this.$route.query["series"]) {
      filterStore.setSeriesFilter(this.$route.query["series"])
    }


  },
  data: function () {
    return {
      uploadModal: {
        active: false,
        id: "",
      },
      sort: {
        key: "title",
        options: [
          "title",
          "author"
        ]
      },
      size: 10,
      page: 0,
      createModalActive: false,
    }
  },
  computed: {
    ...mapState(useBooksStore, ['books']),
  },
  methods: {
    ...mapActions(useBooksStore, ['getBooks', 'createBook', 'uploadBookFiles', 'deleteBook']),
    ...mapActions(useFilesStore, ['deleteFile']),
    clickMenu: function (e) {
      let target = e.currentTarget
      if (target.className.includes("is-active")) {
        target.className = target.className.replace("is-active", "")
      }
      else {
        target.className = e.currentTarget.className + " is-active"
      }
    },
    deleteBookClick: function (bookId) {
      // if (confirm(`Are you sure you want to delete boook ${bookId}?`)) {
      this.deleteBook(bookId)
      // }
    },
    deleteFileClick: function (bookId, fileType) {
      // if (confirm(`Are you you want to delete book ${bookId} file ${fileType}?`)) {
      this.deleteFile(bookId, fileType)
      // }
    },
    handleIntersection: function (entries) {
      if (entries[0].isIntersecting && this.books.length % this.size == 0) {
        this.getBooks(this.page, this.size, this.sort.key)
        this.page++
      }
    },
    async submitCreateBookModal(book, files) {
      let id = await this.createBook(book)
      this.uploadBookFiles(id, files)
      this.toggleCreateModal()

      this.page = 0
      this.getBooks(this.page, this.size, this.sort.key)
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