import assert from 'node:assert/strict'
import test from 'node:test'

import { createSafeStorage } from './safeStorage.ts'

test('falls back to memory when browser storage is blocked by the sandbox', () => {
  const original = Object.getOwnPropertyDescriptor(globalThis, 'localStorage')
  Object.defineProperty(globalThis, 'localStorage', {
    configurable: true,
    get() {
      throw new Error('sandboxed')
    }
  })
  try {
    const storage = createSafeStorage('local')
    assert.equal(storage.getItem('missing'), null)
    storage.setItem('key', 'value')
    assert.equal(storage.getItem('key'), 'value')
    storage.removeItem('key')
    assert.equal(storage.getItem('key'), null)
  } finally {
    if (original) {
      Object.defineProperty(globalThis, 'localStorage', original)
    } else {
      delete globalThis.localStorage
    }
  }
})
