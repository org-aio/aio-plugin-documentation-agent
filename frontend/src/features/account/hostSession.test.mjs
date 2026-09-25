import assert from 'node:assert/strict'
import test from 'node:test'

import { createHostSessionLoader, requestHostSession } from './hostSession.mjs'

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
  assert.equal(request.path, '/admin-api/system/auth/host-login')
  assert.ok(request.body instanceof Uint8Array)
  assert.equal(request.body.length, 0)
  assert.equal(token.accessToken, 'token')
})

test('host login follows a customized API base without producing a double slash', async () => {
  let path
  await requestHostSession(
    {
      request: async (value) => {
        path = value.path
        return {
          status: 200,
          body: new TextEncoder().encode(
            JSON.stringify({ code: 0, data: { accessToken: 'token' } })
          )
        }
      }
    },
    '/tenant-api/'
  )
  assert.equal(path, '/tenant-api/system/auth/host-login')
})

test('host login rejects business and transport failures', async () => {
  assert.equal(await requestHostSession(null), null)
  assert.equal(
    await requestHostSession({
      request: async () => ({
        status: 200,
        body: new TextEncoder().encode(JSON.stringify({ code: 401 }))
      })
    }),
    null
  )
})

test('host session loader shares one in-flight request', async () => {
  let calls = 0
  let release
  const gate = new Promise((resolve) => {
    release = resolve
  })
  const load = createHostSessionLoader({
    request: async () => {
      calls += 1
      await gate
      return {
        status: 200,
        body: new TextEncoder().encode(JSON.stringify({ code: 0, data: { accessToken: 'shared' } }))
      }
    }
  })
  const first = load()
  const second = load()
  while (calls === 0) {
    await Promise.resolve()
  }
  assert.equal(calls, 1)
  release()
  assert.equal((await first).accessToken, 'shared')
  assert.equal((await second).accessToken, 'shared')
  await load()
  assert.equal(calls, 2)
})

test('host session loader resolves a bridge that is injected after startup', async () => {
  let bridge
  const load = createHostSessionLoader(() => bridge)
  bridge = {
    request: async () => ({
      status: 200,
      body: new TextEncoder().encode(
        JSON.stringify({ code: 0, data: { accessToken: 'late-token' } })
      )
    })
  }
  assert.equal((await load()).accessToken, 'late-token')
})
