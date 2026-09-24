import assert from 'node:assert/strict'
import test from 'node:test'
import { safeRedirect, sameSessionIdentity, sessionIsActive } from './session.mjs'

test('跨标签权限刷新保留当前身份，退出或租户切换产生新身份', () => {
  const session = { mode: 'api', accessToken: 'token', tenantId: '1', userId: 1, permissions: [] }
  assert.equal(
    sameSessionIdentity(session, { ...session, permissions: ['read'], roles: ['viewer'] }),
    true
  )
  assert.equal(sameSessionIdentity(session, { ...session, signedOut: true }), false)
  assert.equal(sameSessionIdentity(session, { ...session, tenantId: '2' }), false)
  assert.equal(sameSessionIdentity(session, { ...session, userId: 2 }), false)
})

test('首次访问必须登录，退出标记优先于任何旧令牌，API 不接受演示会话', () => {
  assert.equal(sessionIsActive('demo', {}), false)
  assert.equal(sessionIsActive('demo', { signedOut: true }), false)
  assert.equal(sessionIsActive('api', { accessToken: 'server-token', signedOut: true }), false)
  assert.equal(sessionIsActive('api', { mode: 'demo' }), false)
  assert.equal(sessionIsActive('demo', { mode: 'api', accessToken: 'server-token' }), true)
  assert.equal(sessionIsActive('api', { mode: 'api', accessToken: 'server-token' }), true)
  assert.equal(sessionIsActive('api', {}), false)
})

test('登录回跳仅接受应用内路径并防止登录循环', () => {
  assert.equal(safeRedirect('/system/user?page=2#edit'), '/system/user?page=2#edit')
  for (const value of [
    'https://outside.invalid',
    '//outside.invalid',
    '/%2foutside.invalid',
    '/\\outside.invalid',
    '/%5coutside.invalid',
    '/%0aoutside.invalid',
    '/login?redirect=/login',
    '/../login',
    '/%2e%2e/login',
    '/bad%escape',
    null,
    ['/home']
  ]) {
    assert.equal(safeRedirect(value), '/home', String(value))
  }
})
