<template>
  <modal ref="modal">
    <book-form :id="id" v-bind="book" :allow_files="false" @submit="submit" @cancel="cancel" v-if="id !== 0" />
  </modal>
</template>

<script>
import Modal from '../Modal.vue';
import BookForm from '../BookForm.vue';

import { mapActions } from 'pinia';
import { useBooksStore } from '../../stores/books';

export default {
  components: {
    modal: Modal,
    'book-form': BookForm
  },
  data: function () {
    return {
      id: 0,
      book: {},
    }
  },
  methods: {
    ...mapActions(useBooksStore, ['updateBook']),
    toggle: function () {
      this.$refs.modal.toggle()
    },
    cancel: function () {
      this.book = {}
      this.id = ""

      this.toggle()
    },
    open: function (id, book) {
      this.id = id
      this.book = book

      this.toggle()
    },
    async submit(book, files) {
      await this.updateBook(book.id, book)
      // let id = await this.createBook(book)
      // this.uploadBookFiles(id, files)
      this.toggle()
    },
  }
}
</script>