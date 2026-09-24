import { parseRequest } from '../../features/system-demo/model.mjs'

const toBase64 = (buffer) => {
  const bytes = new Uint8Array(buffer)
  let binary = ''
  for (let offset = 0; offset < bytes.length; offset += 0x8000) {
    binary += String.fromCharCode(...bytes.subarray(offset, offset + 0x8000))
  }
  return globalThis.btoa(binary)
}

const bytesFromBase64 = (value) => {
  const binary = globalThis.atob(value)
  const bytes = new Uint8Array(binary.length)
  for (let index = 0; index < binary.length; index += 1) {
    bytes[index] = binary.charCodeAt(index)
  }
  return bytes
}

const encodeBody = (value) => new TextEncoder().encode(value)
const decodeBody = (value) => {
  if (value instanceof Uint8Array) {
    return new TextDecoder().decode(value)
  }
  return typeof value === 'string' ? value : ''
}

async function serializeFormData(data) {
  if (!(data instanceof FormData)) {
    return data
  }
  const files = []
  const fields = {}
  for (const [key, value] of data.entries()) {
    if (value instanceof Blob) {
      files.push({
        field: key,
        name: value.name || key,
        base64: toBase64(await value.arrayBuffer())
      })
      fields[key] = value.name || key
    } else {
      fields[key] = value
    }
  }
  const primary = files.find((item) => item.field === 'file') || files[0]
  if (primary) {
    return { ...fields, name: primary.name, base64: primary.base64, files }
  }
  return { ...fields, files }
}

export function createRequestClient({
  mode,
  apiBase,
  session,
  demo,
  refreshSession,
  fetcher = globalThis.fetch,
  bridge = globalThis.window?.aioPlugin
}) {
  if (!['demo', 'api'].includes(mode)) {
    throw new Error('dataMode 必须为 demo 或 api。')
  }
  // 无感刷新令牌：并发 401 只触发一次刷新，其余请求等待同一 Promise。
  let refreshing
  const refreshOnce = () => {
    if (!refreshSession) {
      return Promise.resolve(false)
    }
    if (!refreshing) {
      refreshing = Promise.resolve()
        .then(() => refreshSession())
        .then((value) => value !== false)
        .catch(() => false)
        .finally(() => {
          refreshing = undefined
        })
    }
    return refreshing
  }
  const request = async (method, options, kind = 'json', retried = false) => {
    const { path, params } = parseRequest(options)
    const accountPath = path.replace(/^\/+/, '')
    // 业务演示模式也必须通过现有后端验证账号、权限和个人资料。
    const requiresApi =
      options.mode === 'api' ||
      accountPath.startsWith('system/auth/') ||
      accountPath.startsWith('system/captcha/') ||
      accountPath.startsWith('system/user/profile/')
    if (mode === 'demo' && !requiresApi) {
      const result = await demo.request(method, options)
      return kind === 'upload' ? { code: 0, data: result } : result
    }
    const query = new URLSearchParams()
    for (const [key, value] of Object.entries(params)) {
      if (value == null || value === '') {
        continue
      }
      const values = Array.isArray(value) ? value : [value]
      values.forEach((item) => query.append(key, String(item)))
    }
    const bridgeRequest = async (retried = false) => {
      const credentials = session()
      if (credentials.accessToken && !query.has('access_token')) {
        query.set('access_token', credentials.accessToken)
      }
      const requestBody = encodeBody(
        options.data == null ? '' : JSON.stringify(await serializeFormData(options.data))
      )
      const response = await bridge.request({
        method,
        path: `${apiBase.replace(/\/+$/, '')}/${path.replace(/^\/+/, '')}`,
        query: query.toString() || null,
        body: requestBody
      })
      const responseBody = decodeBody(response.body)
      const payload = responseBody ? JSON.parse(responseBody) : null
      if (response.status === 401 && !retried) {
        const refreshed = await refreshOnce()
        if (refreshed) {
          return bridgeRequest(true)
        }
      }
      if (response.status < 200 || response.status >= 300) {
        throw Object.assign(new Error(payload?.msg || payload?.error || `API 请求失败（HTTP ${response.status}）。`), {
          status: response.status,
          code: payload?.code
        })
      }
      if (!payload || typeof payload !== 'object' || !Object.hasOwn(payload, 'code')) {
        throw new Error('API 响应缺少业务状态码 code。')
      }
      if (![0, '0'].includes(payload.code)) {
        if (Number(payload.code) === 401 && !retried) {
          const refreshed = await refreshOnce()
          if (refreshed) {
            return bridgeRequest(true)
          }
        }
        throw Object.assign(new Error(payload.msg || 'API 业务错误。'), { code: payload.code })
      }
      if (kind === 'download') {
        if (payload.data && typeof payload.data === 'object' && typeof payload.data.base64 === 'string') {
          return new Blob([bytesFromBase64(payload.data.base64)], {
            type: payload.data.contentType || 'application/octet-stream'
          })
        }
        throw new Error('API 未返回可下载的文件。')
      }
      return kind === 'original' || kind === 'upload' ? payload : payload.data
    }
    if (bridge && (mode === 'api' || requiresApi)) {
      return bridgeRequest()
    }
    if (!apiBase) {
      throw new Error('API 模式需要配置 apiBase。')
    }
    const suffix = query.toString()
    const url = `${apiBase.replace(/\/+$/, '')}/${path.replace(/^\/+/, '')}${suffix ? `?${suffix}` : ''}`
    const credentials = session()
    const headers = new Headers(options.headers)
    if (credentials.accessToken) {
      headers.set('Authorization', `Bearer ${credentials.accessToken}`)
    }
    if (credentials.tenantId != null && credentials.tenantId !== '') {
      headers.set('tenant-id', String(credentials.tenantId))
    }
    let body
    if (!['GET', 'HEAD'].includes(method) && options.data != null) {
      if (options.data instanceof FormData) {
        body = options.data
        headers.delete('Content-Type')
      } else {
        body = JSON.stringify(options.data)
        headers.set('Content-Type', 'application/json')
      }
    }
    const response = await fetcher(url, { method, headers, body, signal: options.signal })
    if (!response.ok) {
      // 访问令牌过期时先尝试刷新并原样重放；刷新失败或已重试过才抛出。
      if (response.status === 401 && !retried) {
        const refreshed = await refreshOnce()
        if (refreshed) {
          return request(method, options, kind, true)
        }
      }
      throw Object.assign(new Error(`API 请求失败（HTTP ${response.status}）。`), {
        status: response.status
      })
    }
    if (response.status === 204) {
      return kind === 'upload' ? { code: 0, data: null } : null
    }
    const contentType = response.headers.get('content-type') || ''
    if (kind === 'download' && !contentType.includes('json')) {
      return response.blob()
    }
    const payload = await response.json()
    if (kind === 'original') {
      return payload
    }
    if (!payload || typeof payload !== 'object' || !Object.hasOwn(payload, 'code')) {
      throw new Error('API 响应缺少业务状态码 code。')
    }
    if (![0, '0'].includes(payload.code)) {
      if (Number(payload.code) === 401 && !retried) {
        const refreshed = await refreshOnce()
        if (refreshed) {
          return request(method, options, kind, true)
        }
      }
      throw Object.assign(
        new Error(payload.msg || payload.message || `API 业务错误：${payload.code}`),
        { code: payload.code }
      )
    }
    if (kind === 'download') {
      throw new Error('API 未返回可下载的文件。')
    }
    return kind === 'upload' ? payload : payload.data
  }
  return {
    get: (options) => request('GET', options),
    post: (options) => request('POST', options),
    postOriginal: (options) => request('POST', options, 'original'),
    put: (options) => request('PUT', options),
    delete: (options) => request('DELETE', options),
    download: (options) => request('GET', options, 'download'),
    upload: (options) => request('POST', options, 'upload')
  }
}
