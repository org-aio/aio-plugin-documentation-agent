export function createAioEntryResolver(pages, prefix = '/pages/') {
  const routes = new Map(pages.map((page) => [page.id, page.route]))
  return (pathname, search = '') => {
    const normalized = String(pathname).replace(/\/+$/, '')
    const marker = normalized.lastIndexOf(prefix)
    if (marker < 0) {
      return null
    }
    const entry = normalized.slice(marker + prefix.length)
    const id = entry.endsWith('.html') ? entry.slice(0, -'.html'.length) : entry
    const route = routes.get(id)
    if (!route) {
      return null
    }
    const params = new URLSearchParams(search)
    const query = new URLSearchParams()
    for (const [key, value] of params) {
      if (!key.startsWith('__aio_')) {
        query.append(key, value)
      }
    }
    const suffix = query.toString()
    return suffix ? `${route}?${suffix}` : route
  }
}

export function resolveAioDocumentRoute(documentValue, pages) {
  const id = documentValue.querySelector?.('meta[name="aio-page-id"]')?.content
  return (id && pages.find((page) => page.id === id)?.route) || null
}

export function filterMenusByRoutes(menus, routes) {
  const allowed = new Set(routes)
  const resolve = (parentPath, path) => {
    const value = String(path ?? '').trim()
    if (!value || value.includes('://') || value.split('/').some((part) => ['.', '..'].includes(part))) {
      return ''
    }
    const absolute = value.startsWith('/') ? value : `${parentPath}/${value}`
    return absolute.replace(/\/{2,}/g, '/').replace(/\/$/, '') || '/'
  }
  const componentPath = (value) => {
    const component = String(value ?? '').trim()
    if (!component || component.includes('://') || component.split('/').some((part) => ['.', '..'].includes(part))) {
      return ''
    }
    return `/${component.replace(/^\/+/, '')}`.replace(/\/index$/, '') || '/'
  }
  const walk = (rows, parentPath = '') =>
    (rows ?? []).flatMap((row) => {
      const path = resolve(parentPath, row.path)
      const children = walk(row.children ?? [], path)
      const leafAllowed = allowed.has(path) || allowed.has(componentPath(row.component))
      if (!children.length && !leafAllowed) {
        return []
      }
      if (!children.length) {
        return [{ ...row }]
      }
      return [{ ...row, children }]
    })
  return walk(menus)
}
