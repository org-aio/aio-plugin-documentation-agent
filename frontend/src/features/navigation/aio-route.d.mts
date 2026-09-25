export interface AioRoutePage {
  id: string
  route: string
}

export function createAioEntryResolver(
  pages: readonly AioRoutePage[],
  prefix?: string
): (pathname: string, search?: string) => string | null

export function resolveAioDocumentRoute(
  documentValue: Pick<Document, 'querySelector'>,
  pages: readonly AioRoutePage[]
): string | null

export function isAioDocumentEntry(
  documentValue: Pick<Document, 'querySelector'> | null | undefined
): boolean

export function needsAioEntryRedirect(
  to: {
    path: string
    fullPath?: string
    query?: Record<string, unknown>
    hash?: string
  },
  entryRoute: string | null
): boolean

export function shouldUseAioMemoryHistory(windowValue: Window | undefined): boolean

export function isAioEmbedded(windowValue: Window | undefined): boolean

export function filterMenusByRoutes<
  T extends { path?: string; component?: string; children?: T[] }
>(menus: readonly T[], routes: Iterable<string>): T[]
