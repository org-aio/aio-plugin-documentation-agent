import { fileURLToPath, URL } from 'node:url'
import { readdirSync } from 'node:fs'
import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import AutoImport from 'unplugin-auto-import/vite'
import UnoCSS from 'unocss/vite'
import { presetWind3 } from 'unocss'
import appConfig from './app.config.json'

const aioPageInputs = Object.fromEntries(
  readdirSync(fileURLToPath(new URL('./pages', import.meta.url)))
    .filter((name) => name.endsWith('.html'))
    .sort()
    .map((name) => [`pages/${name.slice(0, -'.html'.length)}`, fileURLToPath(new URL(`./pages/${name}`, import.meta.url))])
)

export default defineConfig(({ mode }) => {
  const environment = loadEnv(mode, fileURLToPath(new URL('.', import.meta.url)), 'API_PROXY_')
  const proxy =
    appConfig.apiBase.startsWith('/') && !appConfig.apiBase.startsWith('//')
      ? {
          [appConfig.apiBase]: {
            target: environment.API_PROXY_TARGET || 'http://127.0.0.1:48080',
            changeOrigin: true
          }
        }
      : undefined

  return {
    base: appConfig.basePath,
    plugins: [
      vue(),
      vueJsx(),
      AutoImport({
        imports: [
          'vue',
          'vue-router',
          { '@/hooks/web/useMessage': ['useMessage'], '@/hooks/web/useI18n': ['useI18n'] }
        ],
        dts: false
      }),
      UnoCSS({ presets: [presetWind3()] })
    ],
    resolve: {
      alias: { '@': fileURLToPath(new URL('./src', import.meta.url)) }
    },
    server: { port: 5173, proxy },
    preview: { proxy },
    build: {
      rollupOptions: {
        input: {
          index: fileURLToPath(new URL('./index.html', import.meta.url)),
          ...aioPageInputs
        },
        output: {
          manualChunks: { 'element-plus': ['element-plus'] }
        }
      }
    }
  }
})
