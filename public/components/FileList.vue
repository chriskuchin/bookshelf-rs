<template>
  <div class="downloads">
    <ft v-for="file in files" :key="file.path" :type="getFileFormat(file)" :modifier="getFormatModifier(file)"
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
    getFileFormat: function (file) {
      if (file && file.type) {
        return this.getFileFormatFromType(file.type)
      } else if (file && file.name) {
        getFileFormatFromName(file.name)
      }

      return ""
    },
    getFormatModifier: function (file) {
      if (file && file.type) {
        return this.getFormatModifierFromType(file.type)
      } else if (file && file.name) {
        return getFormatModifierFromName(file.name)
      }

      return ""
    },
    getFileFormatFromName: function (name) {
      let format = this.getTagNameFromType(name)
      let formatParts = format.split(".")

      if (formatParts.length == 1) {
        return format
      } else {
        return formatParts[formatParts.length - 1]
      }
    },
    getFileFormatFromType: function (mime) {
      let format = this.getTagNameFromType(mime)
      let formatParts = format.split(".")

      if (formatParts.length == 1) {
        return format
      } else {
        return formatParts[1]
      }
    },
    getFormatModifierFromName: function (name) {
      let format = this.getTagNameFromName(name)
      let formatParts = format.split(".")

      if (formatParts.length == 2 || formatParts.length == 1) {
        return ""
      } else {
        return formatParts[formatParts.length - 2]
      }
    },
    getFormatModifierFromType: function (mime) {
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
    getDownloadLink: function (book, file) {
      if (file && file.type)
        return "/api/v1/books/" + book.id + "/files/" + this.getTagNameFromType(file.type)

      return ""
    }
  }
}
</script>