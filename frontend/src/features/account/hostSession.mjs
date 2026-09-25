export async function requestHostSession(bridge, apiBase = '/admin-api') {
  const activeBridge = typeof bridge === 'function' ? bridge() : bridge
  if (!activeBridge || typeof activeBridge.request !== 'function') {
    return null
  }
  const path = `${String(apiBase).replace(/\/+$/, '')}/system/auth/host-login`
  const response = await activeBridge.request({
    method: 'POST',
    path,
    query: null,
    body: new Uint8Array()
  })
  const bytes = response?.body
  if (!(bytes instanceof Uint8Array)) {
    throw new Error('宿主登录响应体不是字节数组')
  }
  const payload = bytes.length ? JSON.parse(new TextDecoder().decode(bytes)) : null
  const token = payload?.data
  if (
    response.status < 200 ||
    response.status >= 300 ||
    payload?.code !== 0 ||
    typeof token?.accessToken !== 'string' ||
    !token.accessToken
  ) {
    return null
  }
  return token
}

export function createHostSessionLoader(bridge, apiBase = '/admin-api') {
  let pending
  return () => {
    if (!pending) {
      const operation = requestHostSession(bridge, apiBase)
      pending = operation.finally(() => {
        pending = undefined
      })
    }
    return pending
  }
}
