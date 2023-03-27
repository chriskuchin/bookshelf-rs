import { createApp } from 'vue'
import App from './App.vue'

import { createPinia } from 'pinia'

var pinia = createPinia()
var app = createApp(App).use(pinia)

app.mount('#app')