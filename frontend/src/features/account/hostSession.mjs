export async function requestHostSession(bridge) {
  if (!bridge || typeof bridge.request !== 'function') {
    return null
  }
  const response = await bridge.request({
    method: 'POST',
    path: '/system/auth/host-login',
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
