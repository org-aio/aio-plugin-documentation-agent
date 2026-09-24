import test from 'node:test'
import assert from 'node:assert/strict'
import { createRequestClient } from './transport.mjs'

const apiClient = (
  fetcher,
  demo = {
    request: () => {
      throw new Error('unexpected demo request')
    }
  }
) =>
  createRequestClient({
    mode: 'api',
    apiBase: '/admin-api/',
    session: () => ({ accessToken: 'test-token', tenantId: 7 }),
    demo,
    fetcher
  })

test('API 处理内联参数、数组查询、token 与 tenant 请求头并解包业务数据', async () => {
  const client = apiClient(async (url, options) => {
    assert.equal(
      url,
      '/admin-api/system/user/get?id=2&ids=1&ids=3&createTime=2026-01-01&createTime=2026-02-01'
    )
    assert.equal(options.headers.get('Authorization'), 'Bearer test-token')
    assert.equal(options.headers.get('tenant-id'), '7')
    return Response.json({ code: 0, data: { id: 2 } })
  })
  assert.deepEqual(
    await client.get({
      url: '/system/user/get?id=2',
      params: { ids: [1, 3], createTime: ['2026-01-01', '2026-02-01'], unused: undefined }
    }),
    { id: 2 }
  )
})

test('API 网络、HTTP、业务和协议错误全部拒绝且不回退到演示', async () => {
  let demoCalls = 0
  const demo = {
    request: () => {
      demoCalls += 1
      return []
    }
  }
  const failures = [
    async () => {
      throw new Error('network failed')
    },
    async () => new Response('failure', { status: 503 }),
    async () => Response.json({ code: 401, msg: '认证失效' }),
    async () => Response.json({ data: [] }),
    async () => Response.json({ code: null, data: [] })
  ]
  for (const fetcher of failures) {
    await assert.rejects(apiClient(fetcher, demo).get({ url: '/system/user/page' }))
  }
  assert.equal(demoCalls, 0)
})

test('上传使用 FormData 且保留业务信封，下载检查 JSON 业务错误', async () => {
  const form = new FormData()
  form.append('file', new Blob(['hello']), 'hello.txt')
  const client = apiClient(async (_url, options) => {
    assert.equal(options.body, form)
    assert.equal(options.headers.has('Content-Type'), false)
    return Response.json({ code: 0, data: 'uploaded-file' })
  })
  assert.deepEqual(await client.upload({ url: '/infra/file/upload', data: form }), {
    code: 0,
    data: 'uploaded-file'
  })
  const error = apiClient(async () => Response.json({ code: 403, msg: '不可导出' }))
  await assert.rejects(error.download({ url: '/system/user/export-excel' }), /不可导出/)
})

test('演示模式没有网络请求，上传返回同样的信封；无效模式报错', async () => {
  const client = createRequestClient({
    mode: 'demo',
    apiBase: '',
    session: () => ({}),
    demo: { request: async () => 'local-file' },
    fetcher: () => {
      throw new Error('unexpected network')
    }
  })
  assert.deepEqual(await client.upload({ url: '/infra/file/upload' }), {
    code: 0,
    data: 'local-file'
  })
  assert.throws(() => createRequestClient({ mode: 'unknown' }), /dataMode/)
})

test('认证失效保留 HTTP 与业务状态，调用方能够清除失效会话', async () => {
  const http = apiClient(async () => new Response(null, { status: 401 }))
  await assert.rejects(http.get({ url: '/system/user/profile/get' }), { status: 401 })
  const business = apiClient(async () => Response.json({ code: '401', msg: '登录已过期' }))
  await assert.rejects(business.get({ url: '/system/user/profile/get' }), { code: '401' })
})

test('业务演示模式的登录、权限和个人资料始终请求原后端', async () => {
  const calls = []
  const client = createRequestClient({
    mode: 'demo',
    apiBase: '/admin-api',
    session: () => ({ accessToken: 'backend-token' }),
    demo: {
      request: () => {
        throw new Error('认证接口不得使用本地数据')
      }
    },
    fetcher: async (url, options) => {
      calls.push({ url, method: options.method })
      assert.equal(options.headers.get('Authorization'), 'Bearer backend-token')
      return Response.json({ code: 0, data: true })
    }
  })
  for (const [method, url] of [
    ['post', '/system/auth/login'],
    ['post', '/system/auth/logout'],
    ['get', '/system/auth/get-permission-info'],
    ['get', '/system/captcha/config'],
    ['get', '/system/user/profile/get'],
    ['put', '/system/user/profile/update'],
    ['put', '/system/user/profile/update-password']
  ]) {
    assert.equal(await client[method]({ url }), true)
    assert.deepEqual(calls.at(-1), { url: `/admin-api${url}`, method: method.toUpperCase() })
  }
})

test('业务演示模式验证码保留 Anji 原始响应，失败不会进入演示数据', async () => {
  const calls = []
  const response = { repCode: '6111', repMsg: '验证失败', repData: null }
  const client = createRequestClient({
    mode: 'demo',
    apiBase: '/admin-api',
    session: () => ({}),
    demo: {
      request: () => {
        throw new Error('验证码不得使用本地数据')
      }
    },
    fetcher: async (url) => {
      calls.push(url)
      return Response.json(response)
    }
  })
  for (const url of ['/system/captcha/get', '/system/captcha/check']) {
    assert.deepEqual(await client.postOriginal({ url }), response)
  }
  assert.deepEqual(calls, ['/admin-api/system/captcha/get', '/admin-api/system/captcha/check'])
})

test('业务演示模式显式上传头像到后端，后端失败不回退本地', async () => {
  let demoCalls = 0
  const form = new FormData()
  form.append('file', new Blob(['avatar']), 'avatar.png')
  const options = { url: '/infra/file/upload', mode: 'api', data: form }
  const clientOptions = {
    mode: 'demo',
    apiBase: '/admin-api',
    session: () => ({}),
    demo: {
      request: async () => {
        demoCalls += 1
        return 'local-file'
      }
    }
  }
  const client = createRequestClient({
    ...clientOptions,
    fetcher: async (url, request) => {
      assert.equal(url, '/admin-api/infra/file/upload')
      assert.equal(request.body, form)
      return Response.json({ code: 0, data: '/files/avatar.png' })
    }
  })
  assert.deepEqual(await client.upload(options), { code: 0, data: '/files/avatar.png' })
  const failedClient = createRequestClient({
    ...clientOptions,
    fetcher: async () => new Response(null, { status: 503 })
  })
  await assert.rejects(failedClient.upload(options), { status: 503 })
  await assert.rejects(failedClient.post({ url: '/system/auth/login' }), { status: 503 })
  assert.equal(demoCalls, 0)
})

test('HTTP 401 触发一次刷新并原样重放请求', async () => {
  const seen = []
  let refreshes = 0
  const client = createRequestClient({
    mode: 'api',
    apiBase: '/admin-api',
    session: () => ({ accessToken: 'stale-token' }),
    refreshSession: async () => {
      refreshes += 1
      return true
    },
    demo: {
      request: () => {
        throw new Error('unexpected')
      }
    },
    fetcher: async (url, options) => {
      seen.push(options.headers.get('Authorization'))
      return seen.length === 1
        ? new Response(null, { status: 401 })
        : Response.json({ code: 0, data: 'ok' })
    }
  })
  assert.equal(await client.get({ url: '/system/user/profile/get' }), 'ok')
  assert.equal(refreshes, 1)
  assert.deepEqual(seen, ['Bearer stale-token', 'Bearer stale-token'])
})

test('业务码 401 触发刷新；刷新失败时抛出原始错误', async () => {
  let refreshes = 0
  const recovered = createRequestClient({
    mode: 'api',
    apiBase: '/admin-api',
    session: () => ({ accessToken: 'stale' }),
    refreshSession: async () => {
      refreshes += 1
      return true
    },
    demo: {
      request: () => {
        throw new Error('unexpected')
      }
    },
    fetcher: async () =>
      refreshes === 0
        ? Response.json({ code: 401, msg: '过期' })
        : Response.json({ code: 0, data: 7 })
  })
  assert.equal(await recovered.get({ url: '/system/user/page' }), 7)
  assert.equal(refreshes, 1)

  const failed = createRequestClient({
    mode: 'api',
    apiBase: '/admin-api',
    session: () => ({ accessToken: 'stale' }),
    refreshSession: async () => false,
    demo: {
      request: () => {
        throw new Error('unexpected')
      }
    },
    fetcher: async () => Response.json({ code: 401, msg: '登录已过期' })
  })
  await assert.rejects(failed.get({ url: '/system/user/page' }), { code: 401 })
})

test('并发 401 只触发一次刷新，全部请求重放成功', async () => {
  let refreshes = 0
  const attempts = new Map()
  const client = createRequestClient({
    mode: 'api',
    apiBase: '/admin-api',
    session: () => ({ accessToken: 'stale' }),
    refreshSession: async () => {
      refreshes += 1
      return true
    },
    demo: {
      request: () => {
        throw new Error('unexpected')
      }
    },
    fetcher: async (url) => {
      const count = (attempts.get(url) ?? 0) + 1
      attempts.set(url, count)
      return count === 1
        ? new Response(null, { status: 401 })
        : Response.json({ code: 0, data: url })
    }
  })
  const results = await Promise.all([
    client.get({ url: '/system/user/get?id=1' }),
    client.get({ url: '/system/role/get?id=2' }),
    client.get({ url: '/system/dept/get?id=3' })
  ])
  assert.equal(refreshes, 1)
  assert.deepEqual(results, [
    '/admin-api/system/user/get?id=1',
    '/admin-api/system/role/get?id=2',
    '/admin-api/system/dept/get?id=3'
  ])
})

test('AIO 沙箱通过宿主桥发送 JSON、上传 base64 与还原下载 Blob', async () => {
  const calls = []
  const client = createRequestClient({
    mode: 'api',
    apiBase: '/admin-api',
    session: () => ({ accessToken: 'bridge-token', tenantId: 9 }),
    demo: { request: () => { throw new Error('unexpected') } },
    fetcher: () => { throw new Error('沙箱不得直接 fetch') },
    bridge: {
      request: async (request) => {
        calls.push(request)
        if (request.path.endsWith('/infra/file/upload')) {
          return {
            status: 200,
            body: new TextEncoder().encode(
              JSON.stringify({ code: 0, data: '/infra/file/content?id=3' })
            )
          }
        }
        if (request.path.endsWith('/system/user/export-excel')) {
          return {
            status: 200,
            body: new TextEncoder().encode(
              JSON.stringify({
                code: 0,
                data: { base64: 'aGVsbG8=', contentType: 'application/vnd.ms-excel' }
              })
            )
          }
        }
        return {
          status: 200,
          body: new TextEncoder().encode(JSON.stringify({ code: 0, data: { ok: true } }))
        }
      }
    }
  })
  assert.deepEqual(await client.get({ url: '/system/user/page', params: { pageNo: 2 } }), { ok: true })
  assert.equal(calls[0].path, '/admin-api/system/user/page')
  assert.equal(calls[0].query, 'pageNo=2&access_token=bridge-token')
  const form = new FormData()
  form.append('file', new Blob(['hello']), 'hello.txt')
  form.append('directory', 'avatar')
  assert.deepEqual(await client.upload({ url: '/infra/file/upload', data: form }), {
    code: 0,
    data: '/infra/file/content?id=3'
  })
  assert.ok(calls[0].body instanceof Uint8Array)
  const upload = JSON.parse(new TextDecoder().decode(calls[1].body))
  assert.equal(upload.name, 'hello.txt')
  assert.equal(upload.directory, 'avatar')
  assert.equal(upload.base64, 'aGVsbG8=')
  const blob = await client.download({ url: '/system/user/export-excel' })
  assert.equal(await blob.text(), 'hello')
})
