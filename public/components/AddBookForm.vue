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
            <span class="file-name">
              {{ preview }}
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
import FileList from './FileList.vue';

export default {
  emits: ['cancel', 'submit'],
  components: {
    'fl': FileList,
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
        files: [],
      },
      files: {},
      preview: [],
    }
  },
  methods: {
    fileSelected: function (e) {
      let file = e.target.files[0]

      this.files[file.name] = file

      this.book.files = Object.values(this.files)

      this.preview = []
      Object.values(this.files).forEach(val => {
        this.preview.push({
          name: val.name
        })
      })
    },
    submit: function () {
      this.$emit('submit', this.book)
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
      this.book.files = []
    }
  },
  computed: {
    previewFiles: function () {
      Object.values(this.files).forEach(val => {
        console.log("NAME: ", val.name)
        this.preview.push({
          name: val.name
        })
      })
    }
  }
};
</script>