<template>
  <modal ref="upload-file-modal" @close="reset">
    <uff :bookID="id" @submit="submit" @cancel="toggle" />
  </modal>
</template>

<script>
import { mapActions } from 'pinia';
import { useBooksStore } from '../../stores/books';

import Modal from '../Modal.vue';
import UploadFileForm from '../UploadFileForm.vue'

export default {
  components: {
    modal: Modal,
    uff: UploadFileForm,
  },
  data: function () {
    return {
      id: ""
    }
  },
  methods: {
    ...mapActions(useBooksStore, ['uploadBookFiles']),
    open: function (id) {
      this.id = id
      this.toggle()
    },
    toggle: function () {
      this.$refs['upload-file-modal'].toggle()
    },
    submit: function (id, files) {
      this.uploadBookFiles(id, files)
      this.toggle()
    },
    reset: function () {
      this.id = ""
    },
  }

}
</script>