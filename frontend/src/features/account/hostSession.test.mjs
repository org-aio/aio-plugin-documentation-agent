import assert from 'node:assert/strict'
import test from 'node:test'

import { requestHostSession } from './hostSession.mjs'

test('host login uses the AIO bridge byte protocol and returns a backend token', async () => {
  let request
  const token = await requestHostSession({
    request: async (value) => {
      request = value
      return {
        status: 200,
        body: new TextEncoder().encode(
          JSON.stringify({ code: 0, data: { accessToken: 'token', refreshToken: 'refresh' } })
        )
      }
    }
  })
  assert.equal(request.method, 'POST')
  assert.equal(request.path, '/system/auth/host-login')
  assert.ok(request.body instanceof Uint8Array)
  assert.equal(request.body.length, 0)
  assert.equal(token.accessToken, 'token')
})

test('host login rejects business and transport failures', async () => {
  assert.equal(await requestHostSession(null), null)
  assert.equal(await requestHostSession({ request: async () => ({
    status: 200,
    body: new TextEncoder().encode(JSON.stringify({ code: 401 }))
  }) }), null)
})
