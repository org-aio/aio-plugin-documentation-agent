import { createApp } from 'vue'
import 'element-plus/theme-chalk/dark/css-vars.css'
import './styles/theme.css'
import 'virtual:uno.css'
import { installPageHost } from './compat'
import { router } from './router'
import App from './App.vue'

const app = createApp(App)
installPageHost(app)
app.use(router)
app.mount('#app')
if (globalThis.window.__aioBootstrapTimer) {
  globalThis.window.clearTimeout(globalThis.window.__aioBootstrapTimer)
  delete globalThis.window.__aioBootstrapTimer
}
