export {}

declare global {
  const computed: (typeof import('vue'))['computed']
  const nextTick: (typeof import('vue'))['nextTick']
  const onMounted: (typeof import('vue'))['onMounted']
  const onUnmounted: (typeof import('vue'))['onUnmounted']
  const reactive: (typeof import('vue'))['reactive']
  const readonly: (typeof import('vue'))['readonly']
  const ref: (typeof import('vue'))['ref']
  const unref: (typeof import('vue'))['unref']
  const watch: (typeof import('vue'))['watch']
  const watchEffect: (typeof import('vue'))['watchEffect']
  const useAttrs: (typeof import('vue'))['useAttrs']
  const useSlots: (typeof import('vue'))['useSlots']
  const useRoute: (typeof import('vue-router'))['useRoute']
  const useRouter: (typeof import('vue-router'))['useRouter']
  const useMessage: (typeof import('../hooks/web/useMessage'))['useMessage']
  const useI18n: (typeof import('../hooks/web/useI18n'))['useI18n']
}
