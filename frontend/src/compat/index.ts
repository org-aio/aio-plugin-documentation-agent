import {
  shallowRef,
  watchEffect,
  type App,
  type DirectiveBinding,
  type ShallowRef,
  type WatchStopHandle
} from 'vue'
import ContentWrap from '@/components/ContentWrap.vue'
import Dialog from '@/components/Dialog.vue'
import DictTag from '@/components/DictTag/src/DictTag.vue'
import { Icon } from '@/components/Icon'
import IconSelect from '@/components/IconSelect.vue'
import Pagination from '@/components/Pagination.vue'
import Tooltip from '@/components/Tooltip.vue'
import { checkPermi, checkRole } from '@/utils/permission'

const updatePermission = (
  element: HTMLElement,
  binding: DirectiveBinding<string[]>,
  check: (values: string[]) => boolean
) => {
  const permitted = Array.isArray(binding.value) && check(binding.value)
  element.hidden = !permitted
  if (!permitted) {
    element.style.setProperty('display', 'none', 'important')
  } else {
    element.style.removeProperty('display')
  }
}

export const installPageHost = (app: App) => {
  const components = { ContentWrap, Dialog, DictTag, Icon, IconSelect, Pagination, Tooltip }
  for (const [name, component] of Object.entries(components)) {
    app.component(name, component)
  }
  for (const [name, check] of Object.entries({ hasPermi: checkPermi, hasRole: checkRole })) {
    const effects = new WeakMap<
      HTMLElement,
      {
        binding: ShallowRef<DirectiveBinding<string[]>>
        stop: WatchStopHandle
      }
    >()
    app.directive<HTMLElement, string[]>(name, {
      beforeMount: (element, binding) => {
        const currentBinding = shallowRef(binding)
        // 权限请求与组件渲染独立完成，每个元素单独监听最新权限和指令参数。
        const stop = watchEffect(() => updatePermission(element, currentBinding.value, check), {
          flush: 'sync'
        })
        effects.set(element, { binding: currentBinding, stop })
      },
      updated: (element, binding) => {
        const effect = effects.get(element)
        if (effect) {
          effect.binding.value = binding
        }
      },
      unmounted: (element) => {
        effects.get(element)?.stop()
        effects.delete(element)
      }
    })
  }
}
