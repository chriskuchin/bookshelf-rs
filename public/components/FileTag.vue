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
    getDownloadLink: function () {
      console.log(this.bookID, this.type, this.modifier)
      if (this.bookID) {
        let url = "/api/v1/books/" + this.bookID + "/files/" + this.type
        console.log(url)
        return url
      }

      return null
    }
  }
}
</script>