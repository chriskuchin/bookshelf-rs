<template>
  <div class="downloads">
    <ft v-for="file in files" :key="file.path" :type="getFileFormat(file.type)" :modifier="getFormatModifier(file.type)"
      :bookID="bookID" />
  </div>
</template>


<script>
import FileTag from './FileTag.vue'

export default {
  components: {
    'ft': FileTag
  },
  props: ['files', 'bookID'],
  methods: {
    getFileFormat: function (mime) {
      let format = this.getTagNameFromType(mime)
      let formatParts = format.split(".")

      if (formatParts.length == 1) {
        return format
      } else {
        return formatParts[1]
      }
    },
    getFormatModifier: function (mime) {
      let format = this.getTagNameFromType(mime)
      let formatParts = format.split(".")

      if (formatParts.length == 1) {
        return ""
      } else {
        return formatParts[0]
      }
    },
    getTagNameFromType: function (mime) {
      switch (mime) {
        case "application/x-mobipocket-ebook":
          return "mobi"
        case "application/pdf":
          return "pdf"
        case "application/epub+zip":
          return "epub"
        default:
          return mime
      }
    },
  }
}
</script>