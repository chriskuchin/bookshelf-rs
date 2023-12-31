<template>
  <div class="section" style="padding-top: 15px;">
    <h1 class="title">Authors</h1>
    <div v-for="author in authors" :id="author[0]">
      <router-link :to="`/?author=${author[0]}`">
        {{ author[0] }} ({{ author[1] }})
      </router-link>
    </div>
  </div>
</template>

<script>
import { mapState } from 'pinia';
import { useBooksStore } from '../stores/books';

export default {
  components: {
  },
  mounted: function () {
  },
  data: function () {
    return {}
  },
  computed: {
    ...mapState(useBooksStore, ['authors'])
  },
  methods: {
  }
}
</script>