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

            <div class="navbar-dropdown is-hoverable">
              <a class="navbar-item">
                Author 1
              </a>
              <a class="navbar-item">
                Author 2
              </a>
            </div>
          </div>
          <div class="navbar-item has-dropdown is-hoverable">
            <a class="navbar-link">
              Series
            </a>

            <div class="navbar-dropdown is-hoverable">
              <a class="navbar-item">
                Series 1
              </a>
              <a class="navbar-item">
                Series 2
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

export default {
  components: {
  },
  data: function () {
    return {
      activeBurger: false
    }
  },
  methods: {
    toggleBurger: function () {
      this.activeBurger = !this.activeBurger;
    },
  },
  computed: {}
}
</script>