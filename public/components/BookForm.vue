<template>
  <div>
    <div class="field">
      <label class="label">Title</label>
      <div class="control">
        <input class="input" type="text" placeholder="Title" @blur="getCoverOptions" v-model="book.title">
      </div>
    </div>

    <div class="field mt-4">
      <label class="label">Author</label>
      <div class="control">
        <input class="input" type="text" placeholder="Author" @blur="getCoverOptions" v-model="book.author">
      </div>
    </div>

    <!-- <div class="field">
      <div class="dropdown">
        <div class="dropdown-trigger">
          <button class="button" aria-haspopup="true" aria-controls="cover-selection">
            <span>Content</span>
            <span class="icon is-small">
              <i class="fas fa-angle-down" aria-hidden="true"></i>
            </span>
          </button>
        </div>
        <div class="dropdown-menu" id="dropdown-menu2" role="menu">
          <div class="dropdown-content">
            <div class="dropdown-item">
              <p>
                You can insert <strong>any type of content</strong> within the
                dropdown menu.
              </p>
            </div>
            <hr class="dropdown-divider" />
            <div class="dropdown-item">
              <p>You simply need to use a <code>&lt;div&gt;</code> instead.</p>
            </div>
            <hr class="dropdown-divider" />
            <a href="#" class="dropdown-item"> This is a link </a>
          </div>
        </div>
      </div>
    </div> -->

    <!-- <div class="field">
      <label class="label">Series</label>
      <div class="control">
        <input class="input" type="email" placeholder="Series" v-model="book.series">
      </div>
    </div> -->

    <fi @file="fileSelected" :files="files" v-if="allow_files" class="mt-4" />

    <div class="field mt-4">
      <label class="label">Open Library Cover ID</label>
      <div class="control">
        <input class="input" type="email" placeholder="Series" v-model="book.cover_url">
      </div>
    </div>

    <div v-if="book_options.length > 0">
      <div class="select">
        <select v-model="selected_book">
          <option v-for="(book, idx) in book_options" :value="idx" :key="idx">
            {{ book.title }} ({{ book.first_publish_year }})
          </option>
        </select>
      </div>
      <div class="grid">
        <div class="cell" @click="selectCover(coverId)" v-for="coverId in covers">
          <input type="radio" :value="coverId" v-model="selected_cover" />
          <label :for="coverId"><img :src="getCoverURL(coverId)" /></label>
        </div>
      </div>
    </div>

    <div class="field is-grouped mt-4">
      <div class="control">
        <button class="button is-link" @click="submit">Submit</button>
      </div>
      <div class="control">
        <button class="button is-link is-light" @click="cancel">Cancel</button>
      </div>
    </div>
  </div>
</template>

<script>
import { getCoverURL } from '../api/openlibrary';
import { searchLibrary } from '../api/openlibrary';
import FileInput from './FileInput.vue';

export default {
  emits: ['cancel', 'submit'],
  props: {
    "id": {
      type: Number,
      default: 0,
    },
    "title": {
      type: String,
      default: "",
    },
    "author": {
      type: String,
      default: "",
    },
    "isbn": {
      type: String,
      default: "",
    },
    "description": {
      type: String,
      default: "",
    },
    "cover_url": {
      type: String,
      default: "",
    },
    "publisher": {
      type: String,
      default: "",
    },
    "pub_date": {
      type: String,
      default: "",
    },
    "series": {
      type: String,
      default: "",
    },
    "allow_files": {
      type: Boolean,
      default: true
    }
  },
  components: {
    'fi': FileInput,
  },
  data() {
    console.log(this.id, this.title, this.author)
    return {
      book: {
        id: this.id,
        title: this.title,
        author: this.author,
        isbn: this.isbn,
        description: this.description,
        cover_url: this.cover_url,
        publisher: this.publisher,
        pub_date: this.pub_date,
        series: this.series,
      },
      selected_book: 0,
      selected_cover: "",
      book_options: [],
      cover_options: [],
      files: [],
    }
  },
  watch: {
    selected_cover: function (newVal) {
      console.log("changed", newVal)
    }
  },
  methods: {
    getCoverURL,
    fileSelected: function (files) {
      for (const file of files) {
        this.files.push(file)
      }
    },
    getFileKeyFromName: function (name) {
      return name.substring(name.indexOf(".") + 1)
    },
    getFileFormatFromName: function (name) {
      let formatParts = name.split(".")

      if (formatParts.length == 1) {
        return format
      } else {
        return formatParts[formatParts.length - 1]
      }
    },
    getFormatModifierFromName: function (name) {
      let formatParts = name.split(".")

      if (formatParts.length == 2) {
        return ""
      } else {
        return formatParts[formatParts.length - 2]
      }
    },
    submit: function () {
      this.$emit('submit', {
        id: this.book.id,
        title: this.book.title,
        author: this.book.author,
        isbn: this.book.isbn,
        description: this.book.description,
        cover_url: this.book.cover_url,
        publisher: this.book.publisher,
        pub_date: this.book.pub_date,
        series: this.book.series,
      }, this.files)
      this.clear()
    },
    cancel: function () {
      this.$emit('cancel')
      this.clear()
    },
    clear: function () {
      this.book.title = ""
      this.book.author = ""
      this.book.series = ""
      this.book.cover_url = ""

      this.selected_book = 0

      this.book_options = []
      this.files = []
    },
    selectCover: function (coverId) {
      this.book.cover_url = coverId
    },
    getCoverOptions: async function () {
      if (this.book.title == "" || this.book.author == "")
        return

      this.book_options = (await searchLibrary(this.book.author, this.book.title))['docs']
    }
  },
  computed: {
    covers: function () {
      if (this.book_options.length === 0)
        return []

      return this.book_options[this.selected_book]['edition_key']
    }
  }
};
</script>