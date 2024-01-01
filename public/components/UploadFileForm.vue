<template>
  <div>
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
  props: ['bookID'],
  components: {
    'fi': FileInput,
  },
  data: function () {
    return {
      files: [],
    }
  },
  methods: {
    fileSelected: function (files) {
      console.log(files)
      console.log(files)
      for (const file of files) {
        this.files.push(file)
      }
    },
    submit: function () {
      this.$emit('submit', this.bookID, this.files)
      this.clear()
    },
    cancel: function () {
      this.$emit('cancel')
      this.clear()
    },
    clear: function () {
      this.files = []
    }
  },
  computed: {
  }
};
</script>