import assert from 'node:assert/strict'
import test from 'node:test'
import { sameSessionIdentity, sessionIsActive } from './session.mjs'

test('任意数据模式只接受后端令牌，拒绝旧免密和本地认证标记', () => {
  for (const mode of ['api', 'demo']) {
    assert.equal(sessionIsActive(mode, { mode: 'api', accessToken: 'backend-token' }), true)
    assert.equal(sessionIsActive(mode, { accessToken: 'legacy-backend-token' }), true)
    for (const session of [
      {},
      { mode: 'demo', userId: 1, signedOut: false },
      { mode: 'demo', authenticated: true, userId: 1 },
      { mode: 'demo', accessToken: 'old-demo-marker' },
      { mode: 'api', accessToken: '' },
      { mode: 'api', accessToken: '   ' },
      { mode: 'api', accessToken: 'backend-token', signedOut: true }
    ]) {
      assert.equal(sessionIsActive(mode, session), false, JSON.stringify(session))
    }
  }
})

test('跨标签令牌与账号变更使旧身份失效，权限刷新不切换身份', () => {
  const session = { mode: 'api', accessToken: 'backend-token', userId: 1 }
  assert.equal(sameSessionIdentity(session, { ...session, accessToken: 'new-token' }), false)
  assert.equal(sameSessionIdentity(session, { ...session, userId: 2 }), false)
  assert.equal(sameSessionIdentity(session, { ...session, permissions: ['read'] }), true)
})
