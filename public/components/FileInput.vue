<template>
  <div class="field">
    <label class="label">Book Files</label>
    <div class="control">
      <div class="file has-name is-fullwidth">
        <label class="file-label">
          <input class="file-input" type="file" name="resume" multiple="multiple" v-on:change="fileSelected">
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
</template>

<script>
import FileTag from './FileTag.vue';

export default {
  emits: ['file'],
  props: ['files'],
  components: {
    'ft': FileTag,
  },
  data: function () {
    return {}
  },
  unmounted: function () { },
  methods: {
    fileSelected: function (e) {
      let files = []
      for (let file of e.target.files) {
        files.push({
          ext: this.getFileKeyFromName(file.name),
          file: file
        })
      }

      this.$emit("file", files)
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
  },
  computed: {
    preview() {
      let preview = []
      for (let val of this.files) {
        preview.push({
          name: val.file.name
        })
      }

      return preview
    },
  }
};
</script>