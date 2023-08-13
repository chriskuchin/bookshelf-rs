<template>
  <a class="tags has-addons" :href="getDownloadLink()" download>
    <span v-if="modifier != ''" :class="['tag', 'is-rounded', 'is-dark']">
      {{ modifier }}
    </span>
    <span
      :class="['tag', 'is-rounded', { 'is-primary': type == 'mobi' }, { 'is-info': type == 'pdf' }, { 'is-warning': type == 'epub' }]">
      {{ type }}
    </span>
  </a>
</template>


<script>
export default {
  props: ['type', 'modifier', 'bookID'],
  methods: {
    getFileType: function () {
      if (this.modifier) {
        return this.modifier + "." + this.type
      }

      return this.type
    },
    getDownloadLink: function () {
      if (this.bookID) {
        return "/api/v1/books/" + this.bookID + "/files/" + this.getFileType()
      }

      return null
    }
  }
}
</script>