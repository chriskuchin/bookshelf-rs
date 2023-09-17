<template>
  <div>
    <nav class="navbar is-fixed-top is-transparent" role="navigation" aria-label="main navigation">
      <div class="navbar-brand">
        <a class="navbar-item" href="/">
          <span class="iconify" data-icon="mdi-bookshelf" height="28"></span>
        </a>

        <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbarBasicExample"
          :class="{ 'is-active': activeBurger }" @click="toggleBurger">
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
          <span aria-hidden="true"></span>
        </a>
      </div>

      <div id="navbarBasicExample" class="navbar-menu" :class="{ 'is-active': activeBurger }">
        <div class="navbar-start">
          <div class="navbar-item has-dropdown is-hoverable">
            <a class="navbar-link">
              Authors
            </a>

            <div class="navbar-dropdown is-hoverable is-boxed" v-if="options.authors.length > 0">
              <a class="navbar-item" @click="selectAuthor(author)" v-for="author in options.authors">
                {{ author }}
              </a>
            </div>
          </div>
          <div class="navbar-item has-dropdown is-hoverable is-boxed" v-if="options.series.length > 0">
            <a class="navbar-link">
              Series
            </a>

            <div class="navbar-dropdown is-hoverable">
              <a class="navbar-item" v-for="filter in options.series">
                {{ filter }}
              </a>
            </div>
          </div>
        </div>
        <div class="navbar-end">
          <router-link class="navbar-item" to="/">
            <span class="iconify" data-icon="mdi-rss" height="28"></span>
          </router-link>
        </div>
      </div>
    </nav>
    <router-view></router-view>
  </div>
</template>

<script>
import { mapState, mapActions } from 'pinia'
import { useFiltersStore } from './stores/filters'

export default {
  components: {
  },
  mounted: function () {
    this.getAuthors()
  },
  data: function () {
    return {
      options: {
        authors: [],
        series: [],
      },
      activeBurger: false
    }
  },
  methods: {
    selectAuthor(author) {
      console.log(author)
      this.setAuthorFilter(author)
    },
    async getAuthors() {
      let url = `/api/v1/books/authors`

      let res = await fetch(url)
      this.options.authors = await res.json()
    },
    toggleBurger: function () {
      this.activeBurger = !this.activeBurger;
    },
    ...mapActions(useFiltersStore, ['setAuthorFilter']),
  },
  computed: {
    ...mapState(useFiltersStore, ['author', 'series', 'title'])
  }
}
</script>