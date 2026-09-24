<script setup lang="ts">
import { nextTick, ref, watch } from 'vue'
import { ArrowDown, ArrowLeft, ArrowRight, Close, RefreshRight } from '@element-plus/icons-vue'
import { t } from '@/locales'
import { iconFor } from './menuIcons'

defineOptions({ name: 'TagsView' })
const props = defineProps<{
  tabs: { path: string; title: string; icon: string }[]
  activePath: string
  showIcons: boolean
}>()
const emit = defineEmits<{
  close: [path: string]
  action: [command: string, path: string]
}>()
const scroller = ref<HTMLElement>()
const scroll = (direction: number) => {
  scroller.value?.scrollBy({ left: direction * 240, behavior: 'smooth' })
}
watch(
  () => props.activePath,
  async () => {
    await nextTick()
    const active = scroller.value?.querySelector<HTMLElement>('.is-active')
    if (!active || !scroller.value) return
    const left = active.offsetLeft - scroller.value.offsetLeft
    if (
      left < scroller.value.scrollLeft ||
      left + active.offsetWidth > scroller.value.scrollLeft + scroller.value.clientWidth
    ) {
      scroller.value.scrollTo({ left: Math.max(0, left - 24), behavior: 'smooth' })
    }
  }
)
const actions = ['refresh', 'closeCurrent', 'closeOther', 'closeLeft', 'closeRight', 'closeAll']
</script>

<template>
  <nav class="tags-view" :aria-label="t('app.tabs')">
    <button class="tags-tool-button" :aria-label="t('tabs.scrollLeft')" @click="scroll(-1)">
      <ElIcon><ArrowLeft /></ElIcon>
    </button>
    <div ref="scroller" class="tags-scroll">
      <ElDropdown
        v-for="tab in tabs"
        :key="tab.path"
        trigger="contextmenu"
        @command="(command: string) => emit('action', command, tab.path)"
      >
        <div class="page-tab" :class="{ 'is-active': activePath === tab.path }">
          <RouterLink :to="tab.path" :aria-current="activePath === tab.path ? 'page' : undefined">
            <ElIcon v-if="showIcons" class="tab-icon"><component :is="iconFor(tab.icon)" /></ElIcon
            >{{ tab.title }}
          </RouterLink>
          <button
            v-if="tab.path !== '/home'"
            :aria-label="`${t('app.closeTab')} ${tab.title}`"
            @click="emit('close', tab.path)"
          >
            <ElIcon><Close /></ElIcon>
          </button>
        </div>
        <template #dropdown>
          <ElDropdownMenu>
            <ElDropdownItem
              v-for="command in actions"
              :key="command"
              :command="command"
              :disabled="command === 'closeCurrent' && tab.path === '/home'"
            >
              {{ t(`tabs.${command}`) }}
            </ElDropdownItem>
          </ElDropdownMenu>
        </template>
      </ElDropdown>
    </div>
    <button class="tags-tool-button" :aria-label="t('tabs.scrollRight')" @click="scroll(1)">
      <ElIcon><ArrowRight /></ElIcon>
    </button>
    <button
      class="tags-tool-button"
      :aria-label="t('tabs.refresh')"
      @click="emit('action', 'refresh', activePath)"
    >
      <ElIcon><RefreshRight /></ElIcon>
    </button>
    <ElDropdown @command="(command: string) => emit('action', command, activePath)">
      <button class="tags-tool-button" :aria-label="t('tabs.actions')">
        <ElIcon><ArrowDown /></ElIcon>
      </button>
      <template #dropdown>
        <ElDropdownMenu>
          <ElDropdownItem
            v-for="command in actions"
            :key="command"
            :command="command"
            :disabled="command === 'closeCurrent' && activePath === '/home'"
          >
            {{ t(`tabs.${command}`) }}
          </ElDropdownItem>
        </ElDropdownMenu>
      </template>
    </ElDropdown>
  </nav>
</template>
