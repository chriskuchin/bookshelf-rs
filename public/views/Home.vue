<template>
  <div class="section" ref="wrapper" style="padding-top: 15px;">
    <div class="fixed-grid has-9-cols">
      <div class="grid">
        <div class="cell" style="width: 180px; min-height: 250px;" v-for="book in books">
          <div class="" style="position: relative; height: 100%;">
            <div class="dropdown is-right" style="position: absolute; top: 0; right: 0;" @click="clickMenu">
              <div class="dropdown-trigger">
                <span class="icon" aria-haspopup="true" aria-controls="dropdown-menu">
                  <icon icon="fa-solid fa-ellipsis-v"></icon>
                </span>
              </div>
              <div class="dropdown-menu" id="dropdown-menu" role="menu">
                <div class="dropdown-content">
                  <div class="dropdown-item">
                    {{ book.id }}. {{ book.title }}
                    <br />
                    {{ book.author }}
                    <br />
                    {{ book.files }}
                  </div>
                  <hr class="dropdown-divider" />
                  <a class="dropdown-item" @click="openUploadModal(book.id)">
                    <icon icon="fa-solid fa-upload"></icon>
                    Upload Files
                  </a>
                  <a class="dropdown-item" @click="openEditBookModal(book.id, book)">Edit Book</a>
                  <hr class="dropdown-divider" />
                  <a class="dropdown-item" @click="deleteBookClick(book.id)">Delete Book</a>
                  <hr v-if="book.files.length > 0" class="dropdown-divider" />
                  <a v-for="file in book.files" class="dropdown-item" @click="deleteFileClick(book.id, file.type)">
                    Delete {{ file.type }}
                  </a>
                </div>
              </div>
            </div>
            <!-- <fl :files="book.files" :bookID="book.id" /> -->

            <img :title="`${book.title} by ${book.author}`" :alt="`${book.title} by ${book.author}`"
              :src="getCoverURL(book.cover_url)" v-if="book.cover_url != ''" />
            <div class="has-background-primary has-text-primary-invert has-radius-normal"
              style="height: 270px; position: relative; display: block; z-index: -1;" v-else>
              <div class="is-size-5 has-text-centered pt-4">{{ book.title }}</div>
              <div class="is-size-6 has-text-centered pt-1">{{ book.author }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="fixed-bottom">
      <a class="button is-primary is-large fab" @click="toggleCreateModal">
        <icon icon="fa-solid fa-plus"></icon>
      </a>
    </div>

    <div :class="['modal', { 'is-active': createModalActive }]" ref="create-modal">
      <div class="modal-background" @click="toggleCreateModal"></div>
      <div class="modal-content">
        <div class="box">
          <abf @submit="submitCreateBookModal" @cancel="toggleCreateModal" />
        </div>
      </div>
    </div>
    <ebm ref="edit-book-modal" />
    <ufm ref="upload-file-modal" />
    <div id="sentinel" ref="sentinel" style="height: 10px;"></div>
  </div>
</template>

<script>
import AddBookForm from '../components/BookForm.vue'
import UploadFileModal from '../components/modals/UploadFile.vue'
import EditBookModal from '../components/modals/EditBook.vue'
import FileList from '../components/FileList.vue'
import { searchLibrary, getCoverURL } from '../api/openlibrary'
import { mapActions, mapState } from 'pinia';
import { useBooksStore } from '../stores/books';
import { useFiltersStore } from '../stores/filters';
import { useFilesStore } from '../stores/files';

export default {
  components: {
    abf: AddBookForm,
    ufm: UploadFileModal,
    fl: FileList,
    ebm: EditBookModal,
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
      size: 50,
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
    getCoverURL,
    getBooksInfo: function (author, title) {
      searchLibrary(author, title).then((info) => {
        const book = info['docs'][0]
        console.log(book)
      })
    },
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
    openUploadModal: function (bookId) {
      this.$refs['upload-file-modal'].open(bookId)
    },
    openEditBookModal: function (id, book) {
      console.log(book)
      this.$refs['edit-book-modal'].open(id, book)
    }
  }
}
</script>