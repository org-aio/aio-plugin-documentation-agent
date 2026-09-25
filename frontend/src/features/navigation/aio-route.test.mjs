import assert from 'node:assert/strict'
import test from 'node:test'

import {
  createAioEntryResolver,
  filterMenusByRoutes,
  isAioEmbedded,
  isAioDocumentEntry,
  needsAioEntryRedirect,
  resolveAioDocumentRoute,
  shouldUseAioMemoryHistory
} from './aio-route.mjs'

const resolve = createAioEntryResolver([
  { id: 'menu-5', route: '/home' },
  { id: 'menu-22', route: '/boxun/project-info' }
])

test('resolves AIO page entries from both hosted and direct paths', () => {
  assert.equal(
    resolve('/api/runtime/components/assets/token/pages/menu-22.html', ''),
    '/boxun/project-info'
  )
  assert.equal(resolve('/pages/menu-5.html', '?__aio_prepare=1'), '/home')
})

test('preserves business query parameters and ignores host preparation flags', () => {
  assert.equal(
    resolve('/pages/menu-22.html', '?__aio_prepare=1&projectId=42'),
    '/boxun/project-info?projectId=42'
  )
  assert.equal(resolve('/pages/unknown.html', ''), null)
})

test('resolves the route from the generated entry metadata after the host rewrites the URL', () => {
  const documentValue = {
    querySelector: (selector) =>
      selector === 'meta[name="aio-page-id"]' ? { content: 'menu-22' } : null
  }
  assert.equal(
    resolveAioDocumentRoute(documentValue, [{ id: 'menu-22', route: '/boxun/project-info' }]),
    '/boxun/project-info'
  )
})

test('does not redirect a home entry back to itself', () => {
  assert.equal(
    needsAioEntryRedirect({ path: '/home', fullPath: '/home', query: {} }, '/home'),
    false
  )
  assert.equal(
    needsAioEntryRedirect(
      {
        path: '/boxun/project-info',
        fullPath: '/boxun/project-info',
        query: {}
      },
      '/boxun/project-info'
    ),
    false
  )
  assert.equal(
    needsAioEntryRedirect({ path: '/home', fullPath: '/home', query: {} }, '/boxun/project-info'),
    true
  )
  assert.equal(
    needsAioEntryRedirect({ path: '/home', fullPath: '/home', query: {} }, '/home?projectId=42'),
    true
  )
  assert.equal(
    needsAioEntryRedirect(
      {
        path: '/home',
        fullPath: '/home?projectId=42',
        query: { projectId: '42' }
      },
      '/home?projectId=42'
    ),
    false
  )
})

test('uses memory history only for AIO or opaque-origin embedded windows', () => {
  assert.equal(shouldUseAioMemoryHistory({ aioPlugin: {} }), true)
  assert.equal(shouldUseAioMemoryHistory({ location: { origin: 'null' } }), true)
  assert.equal(shouldUseAioMemoryHistory({ location: { origin: 'https://example.test' } }), false)
  assert.equal(shouldUseAioMemoryHistory(undefined), false)
})

test('detects the AIO sandbox before the host bridge is injected', () => {
  assert.equal(
    isAioEmbedded({
      location: { pathname: '/api/runtime/components/assets/token/pages/menu-5.html' },
      parent: {}
    }),
    true
  )
  assert.equal(
    isAioEmbedded({
      location: { pathname: '/', origin: 'null' },
      parent: {}
    }),
    true
  )
  assert.equal(
    isAioEmbedded({
      location: { pathname: '/', origin: 'https://example.test' },
      parent: {}
    }),
    true
  )
  assert.equal(
    isAioEmbedded({
      location: { pathname: '/', origin: 'https://example.test' },
      parent: undefined
    }),
    false
  )
})

test('uses the generated page marker before the parent bridge exists', () => {
  const documentValue = {
    querySelector: (selector) =>
      selector === 'meta[name="aio-page-id"]' ? { content: 'menu-25' } : null
  }
  assert.equal(isAioDocumentEntry(documentValue), true)
  assert.equal(
    isAioEmbedded({
      document: documentValue,
      location: { pathname: '/pages/menu-25.html', origin: 'https://aio.addzero.site' },
      parent: undefined
    }),
    true
  )
})

test('keeps only business routes and their parent groups', () => {
  const menus = [
    {
      id: 1,
      path: '/system',
      children: [{ id: 2, path: 'user', component: 'system/user/index' }]
    },
    {
      id: 10,
      path: '/boxun/raw-data',
      children: [
        {
          id: 11,
          path: 'commercial-concrete-ledger',
          component: 'boxun/commercial-concrete-ledger/index'
        }
      ]
    },
    { id: 20, path: '/boxun/project-info', component: 'boxun/project-info/index' }
  ]
  assert.deepEqual(
    filterMenusByRoutes(menus, ['/boxun/commercial-concrete-ledger', '/boxun/project-info']),
    [
      {
        id: 10,
        path: '/boxun/raw-data',
        children: [
          {
            id: 11,
            path: 'commercial-concrete-ledger',
            component: 'boxun/commercial-concrete-ledger/index'
          }
        ]
      },
      { id: 20, path: '/boxun/project-info', component: 'boxun/project-info/index' }
    ]
  )
})
