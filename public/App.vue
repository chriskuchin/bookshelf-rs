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

          <router-link to="/authors" class="navbar-item">
            Authors
          </router-link>

          <router-link to="/series" class="navbar-item" v-if="series.length > 0">
            Series
          </router-link>

        </div>
      </div>
      <div class="navbar-end">

        <div class="navbar-item">
          <div class="field has-addons">
            <div class="control">
              <input class="input is-small" type="text" placeholder="Title Search" v-model="inputs.title"
                @keypress.enter="searchTitles">
            </div>
            <div class="control">
              <a class="button is-info is-small" @click="searchTitles">
                Search
              </a>
            </div>
          </div>

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
import { useBooksStore } from './stores/books'

export default {
  components: {
  },
  mounted: function () {
    this.getAuthors()
    this.getSeries()
  },
  data: function () {
    return {
      inputs: {
        title: "",
      },
      activeBurger: false
    }
  },
  methods: {
    ...mapActions(useBooksStore, ["getAuthors", "getSeries"]),
    selectAuthor(author) {
      this.setAuthorFilter(author)
    },
    selectSeries(series) {
      this.setSeriesFilter(series)
    },
    searchTitles() {
      this.setTitleFilter(this.inputs.title)
    },
    toggleBurger: function () {
      this.activeBurger = !this.activeBurger;
    },
    ...mapActions(useFiltersStore, ['setAuthorFilter', 'setTitleFilter', 'setSeriesFilter']),
  },
  computed: {
    ...mapState(useFiltersStore, ['isSelectedAuthor', 'isSelectedSeries']),
    ...mapState(useBooksStore, ['authors', 'series'])
  }
}
</script>