import assert from 'node:assert/strict'
import test from 'node:test'

import { createAioEntryResolver, resolveAioDocumentRoute } from './aio-route.mjs'

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
