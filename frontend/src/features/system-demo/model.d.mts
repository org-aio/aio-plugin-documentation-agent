export const DEMO_NOTICE: string
export interface DemoRequestOptions {
  url: string
  params?: object
  data?: unknown
}
export interface DemoTransport {
  request(method: string, options: DemoRequestOptions): Promise<unknown>
}
export function parseRequest(options: DemoRequestOptions): {
  path: string
  params: Record<string, unknown>
  data: unknown
}
export interface MenuCatalogState {
  menuCatalogVersion?: number
  nextId: number
  menus: import('../navigation/catalog.mjs').MenuRecord[]
  roles: Array<{ id: number; code: string }>
  roleMenus: Record<string, Array<number | string>>
}
export function migrateMenuCatalog(state: MenuCatalogState): boolean
export function createDemoTransport(options: {
  storage: Pick<Storage, 'getItem' | 'setItem'>
  storageKey: string
}): DemoTransport
