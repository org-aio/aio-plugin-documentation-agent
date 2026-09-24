import * as icons from '@element-plus/icons-vue'
import type { Component } from 'vue'

const toIconName = (name: string) =>
  `ep:${name
    .replace(/([A-Z]+)([A-Z][a-z])/g, '$1-$2')
    .replace(/([a-z0-9])([A-Z])/g, '$1-$2')
    .toLowerCase()}`

export const iconOptions = Object.keys(icons).map(toIconName)

export const resolveIcon = (name: string): Component => {
  const shortName = name.replace(/^ep:/, '')
  const componentName = shortName.replace(/(^|-)([a-z])/g, (_match, _prefix, letter: string) =>
    letter.toUpperCase()
  )
  return (icons as Record<string, Component>)[componentName] ?? icons.Document
}
