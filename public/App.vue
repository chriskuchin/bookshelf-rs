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

            <div class="navbar-dropdown is-hoverable is-boxed" v-if="options.authors.length > 0"
              style="overflow-y: auto;">
              <a class="navbar-item" @click="selectAuthor('')">
                <icon icon="fa-solid fa-check" v-if="isSelectedAuthor('')" class="mr-2"></icon>
                <span v-else class="mr-2" style="display: inline-block; width: 12.25px;"></span>
                None
              </a>
              <hr class="dropdown-divider">
              <a class="navbar-item" @click="selectAuthor(author)" v-for="author in options.authors">
                <icon icon="fa-solid fa-check" class="mr-2" v-if="isSelectedAuthor(author)"></icon>
                <span v-else class="mr-2" style="display: inline-block; width: 12.25px;"></span>
                {{ author }}
              </a>
            </div>
          </div>
          <div class="navbar-item has-dropdown is-hoverable is-boxed" v-if="options.series.length > 0">
            <a class="navbar-link">
              Series
            </a>
            <div class="navbar-dropdown is-hoverable">
              <a class="navbar-item" @click="selectSeries('')">
                <icon icon="fa-solid fa-check" v-if="isSelectedSeries('')" class="mr-2"></icon>
                <span v-else class="mr-2" style="display: inline-block; width: 12.25px;"></span>
                None
              </a>
              <hr class="dropdown-divider">
              <a class="navbar-item" v-for="series in options.series" @click="selectSeries(series[0])">
                <icon icon="fa-solid fa-check" v-if="isSelectedSeries(series[0])" class="mr-2"></icon>
                <span v-else class="mr-2" style="display: inline-block; width: 12.25px;"></span>
                {{ series[1] }}
              </a>
            </div>
          </div>
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
    this.getSeries()
  },
  data: function () {
    return {
      options: {
        authors: [],
        series: [],
      },
      inputs: {
        title: "",
      },
      activeBurger: false
    }
  },
  methods: {
    selectAuthor(author) {
      this.setAuthorFilter(author)
    },
    async getAuthors() {
      let url = `/api/v1/books/authors`

      let res = await fetch(url)
      this.options.authors = await res.json()
    },
    selectSeries(series) {
      this.setSeriesFilter(series)
    },
    async getSeries() {
      let url = `/api/v1/books/series`
      let res = await fetch(url)

      if (res.ok) {
        this.options.series = await res.json()
      }

      return []
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
    ...mapState(useFiltersStore, ['author', 'series', 'title', 'isSelectedAuthor', 'isSelectedSeries'])
  }
}
</script>