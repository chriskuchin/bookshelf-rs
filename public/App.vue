<template>
  <div>
    <v-app>
      <v-app-bar color="blue darken-3" density="compact" :elevation="0">
        <v-avatar size="32px" item>
          <v-icon icon="$bookshelf" large></v-icon>
        </v-avatar>
        <v-app-bar-title>BookShelf</v-app-bar-title>
      </v-app-bar>
      <router-view></router-view>
    </v-app>
  </div>
</template>

<script>
import { VAppBar, VAppBarTitle, VIcon, VAvatar } from 'vuetify/components'

export default {
  components: {
    VAppBar,
    VAppBarTitle,
    VIcon,
    VAvatar,
  },
  data: function () {
    return {
    }
  },
  computed: {}
}
</script>