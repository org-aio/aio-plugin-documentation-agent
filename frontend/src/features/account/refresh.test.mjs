import assert from 'node:assert/strict'
import test from 'node:test'
import { createSessionRefresher } from './refresh.mjs'

test('刷新令牌成功后回调写入新的访问令牌并返回 true', async () => {
  const calls = []
  let written
  const refresh = createSessionRefresher({
    apiBase: '/admin-api/',
    getRefreshToken: () => 'refresh-abc',
    onRefreshed: (accessToken, refreshToken) => {
      written = { accessToken, refreshToken }
    },
    fetcher: async (url, options) => {
      calls.push({ url, method: options.method })
      return Response.json({
        code: 0,
        data: { accessToken: 'new-access', refreshToken: 'new-refresh' }
      })
    }
  })
  assert.equal(await refresh(), true)
  assert.deepEqual(calls, [
    { url: '/admin-api/system/auth/refresh-token?refreshToken=refresh-abc', method: 'POST' }
  ])
  assert.deepEqual(written, { accessToken: 'new-access', refreshToken: 'new-refresh' })
})

test('缺少刷新令牌或后端拒绝时不写会话并返回 false', async () => {
  const noToken = createSessionRefresher({
    apiBase: '/admin-api',
    getRefreshToken: () => '',
    onRefreshed: () => {
      throw new Error('不应写入')
    },
    fetcher: () => {
      throw new Error('不应发起请求')
    }
  })
  assert.equal(await noToken(), false)

  for (const fetcher of [
    async () => new Response(null, { status: 401 }),
    async () => Response.json({ code: 401, msg: '刷新令牌已过期' }),
    async () => Response.json({ code: 0, data: {} }),
    async () => {
      throw new Error('network down')
    }
  ]) {
    const refresh = createSessionRefresher({
      apiBase: '/admin-api',
      getRefreshToken: () => 'refresh-abc',
      onRefreshed: () => {
        throw new Error('不应写入')
      },
      fetcher
    })
    assert.equal(await refresh(), false)
  }
})
