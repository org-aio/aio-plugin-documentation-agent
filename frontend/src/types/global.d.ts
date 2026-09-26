declare global {
  interface Window {
    __aioBootstrapTimer?: number
  }

  type Nullable<T> = T | null
  type Recordable<T = unknown> = Record<string, T>
  type DictDataType = import('../utils/dict').DictDataType

  interface PageParam {
    pageNo: number
    pageSize: number
  }

  interface Tree {
    id: number | string
    name: string
    label?: string
    parentId?: number | string
    children?: Tree[]
  }
}

declare module 'vue' {
  export interface GlobalComponents {
    ContentWrap: (typeof import('../components/ContentWrap.vue'))['default']
    Dialog: (typeof import('../components/Dialog.vue'))['default']
    DictTag: (typeof import('../components/DictTag/src/DictTag.vue'))['default']
    Icon: (typeof import('../components/Icon/Icon.vue'))['default']
    IconSelect: (typeof import('../components/IconSelect.vue'))['default']
    Pagination: (typeof import('../components/Pagination.vue'))['default']
    Tooltip: (typeof import('../components/Tooltip.vue'))['default']
  }
}

export {}
