import { createApp } from 'vue'
import App from './App.vue'

import { createPinia } from 'pinia'
import router from './routes'

import('./css/bookshelf.scss')

// Vuetify
import('vuetify/styles')
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { aliases, mdi } from 'vuetify/iconsets/mdi-svg'
import { mdiBookshelf } from '@mdi/js'



const vuetify = createVuetify({
  icons: {
    defaultSet: 'mdi',
    aliases: {
      ...aliases,
      bookshelf: mdiBookshelf,
    },
    sets: {
      mdi,
    }
  },
  components,
  directives,
})

var pinia = createPinia()
createApp(App).use(pinia).use(router).use(vuetify).mount('#app')
