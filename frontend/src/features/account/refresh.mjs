// 使用刷新令牌换取新的访问令牌；只依赖 fetch，避免与请求客户端形成循环引用。
export function createSessionRefresher({
  apiBase,
  getRefreshToken,
  onRefreshed,
  fetcher = globalThis.fetch
}) {
  return async () => {
    const refreshToken = getRefreshToken()
    if (!apiBase || !refreshToken) {
      return false
    }
    const url = `${apiBase.replace(/\/+$/, '')}/system/auth/refresh-token?refreshToken=${encodeURIComponent(refreshToken)}`
    let response
    try {
      response = await fetcher(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' }
      })
    } catch {
      return false
    }
    if (!response.ok) {
      return false
    }
    let payload
    try {
      payload = await response.json()
    } catch {
      return false
    }
    const data = payload && typeof payload === 'object' ? payload.data : null
    const accessToken = data && typeof data.accessToken === 'string' ? data.accessToken.trim() : ''
    if (![0, '0'].includes(payload?.code) || !accessToken) {
      return false
    }
    onRefreshed(accessToken, typeof data.refreshToken === 'string' ? data.refreshToken : undefined)
    return true
  }
}
