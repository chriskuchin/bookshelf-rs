import { createApp } from 'vue'
import App from './App.vue'

import { createPinia } from 'pinia'
import router from './routes'

import('./css/bookshelf.scss')

/* import the fontawesome core */
import { library } from '@fortawesome/fontawesome-svg-core'

/* import font awesome icon component */
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

/* import specific icons */
import { faPlus, faCheck } from '@fortawesome/free-solid-svg-icons'

library.add(faPlus, faCheck)

var pinia = createPinia()
createApp(App)
  .component('icon', FontAwesomeIcon)
  .use(pinia).use(router).mount('#app')

require('./assets/')
