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

    <div class="field">
      <label class="label">Book File</label>
      <div class="control">
        <div class="file has-name is-fullwidth">
          <label class="file-label">
            <input class="file-input" type="file" name="resume" v-on:change="fileSelected">
            <span class="file-cta">
              <span class="file-icon">
                <i class="fas fa-upload"></i>
              </span>
              <span class="file-label">
                Choose a file…
              </span>
            </span>
            <span class="file-name tags downloads">
              <ft v-for="file in preview" :key="file.name" :type="getFileFormatFromName(file.name)"
                :modifier="getFormatModifierFromName(file.name)" />
            </span>
          </label>
        </div>
      </div>
    </div>

    <!-- <fl :files="preview" bookID="other" /> -->
    <!-- <div class="field">
      <label class="label">Message</label>
      <div class="control">
        <textarea class="textarea" placeholder="Textarea"></textarea>
      </div>
    </div> -->

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
import FileTag from './FileTag.vue';

export default {
  emits: ['cancel', 'submit'],
  components: {
    'ft': FileTag,
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
      files: {},
      preview: [],
    }
  },
  methods: {
    fileSelected: function (e) {
      let file = e.target.files[0]

      this.files[this.getFileKeyFromName(file.name)] = file

      this.preview = []
      Object.values(this.files).forEach(val => {
        this.preview.push({
          name: val.name
        })
      })
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
    }
  },
  computed: {
    previewFiles: function () {
      Object.values(this.files).forEach(val => {
        this.preview.push({
          name: val.name
        })
      })
    }
  }
};
</script>