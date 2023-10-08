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
  emits: ['cancel', 'submit', 'files'],
  props: ['bookID'],
  components: {
    'fi': FileInput,
  },
  data: function () {
    return {
      files: {},
    }
  },
  methods: {
    fileSelected: function (files) {
      this.files = files
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
      this.files = {}
    }
  },
  computed: {
  }
};
</script>