<template>
  <div class="section" style="padding-top: 15px;">
    <h1 class="title">Series</h1>

    <div v-for="s in series" :id="s[0]">
      <router-link :to="`/?series=${s[1]}`">
        {{ s[1] }}
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
    ...mapState(useBooksStore, ['series'])
  },
  methods: {
  }
}
</script>