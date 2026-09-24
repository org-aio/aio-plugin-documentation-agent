export interface PageMetadata {
  titleKey: string
  icon: string
  order: number
  group?: string
  groupTitleKey?: string
  hidden?: boolean
}

export interface MenuRecord {
  id: number | string
  name: string
  path: string
  parentId?: number | string
  permission?: string
  type?: number
  sort?: number
  status?: number
  visible?: boolean
  icon?: string
  component?: string
  keepAlive?: boolean
  alwaysShow?: boolean
  createTime?: string
  children?: MenuRecord[]
}

export interface NavigationItem {
  path: string
  title: string
  icon: string
  children?: NavigationItem[]
}

export const MENU_CATALOG_VERSION: number
export const pageMetadata: Readonly<Record<string, Readonly<PageMetadata>>>
export function createDefaultMenus(): MenuRecord[]
export function buildNavigation(
  menus: readonly MenuRecord[],
  availablePaths: Iterable<string>
): NavigationItem[]
export function collectAccessiblePaths(
  menus: readonly MenuRecord[] | null | undefined,
  availablePaths: Iterable<string>
): Set<string> | null
export function collectNoCachePaths(
  menus: readonly MenuRecord[] | null | undefined,
  availablePaths?: Iterable<string>
): Set<string>
