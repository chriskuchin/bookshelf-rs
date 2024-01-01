<template>
  <div>
    <div class="field">
      <label class="label">Title</label>
      <div class="control">
        <input class="input" type="text" placeholder="Title" v-model="book.title">
      </div>
    </div>

    <div class="field">
      <label class="label">Author</label>
      <div class="control">
        <input class="input" type="text" placeholder="Author" v-model="book.author">
      </div>
    </div>

    <!-- <div class="field">
      <label class="label">Series</label>
      <div class="control">
        <input class="input" type="email" placeholder="Series" v-model="book.series">
      </div>
    </div> -->

    <fi @file="fileSelected" :files="files" />

    <div class="field is-grouped">
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
import FileInput from './FileInput.vue';

export default {
  emits: ['cancel', 'submit'],
  components: {
    'fi': FileInput,
  },
  data: function () {
    return {
      book: {
        title: "",
        author: "",
        isbn: "",
        description: "",
        cover_url: "",
        publisher: "",
        pub_date: "",
        series: "",
      },
      files: [],
    }
  },
  methods: {
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
      this.$emit('submit', this.book, this.files)
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
      this.files = []
    }
  },
  computed: {}
};
</script>