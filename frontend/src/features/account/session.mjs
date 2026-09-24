// 只允许应用内的绝对路径，防止登录回跳成为开放重定向。
export function safeRedirect(value, fallback = '/home') {
  if (typeof value !== 'string' || !value.startsWith('/') || /[\\\u0000-\u0020]/.test(value)) {
    return fallback
  }
  let decoded
  try {
    decoded = decodeURIComponent(value)
  } catch {
    return fallback
  }
  if (decoded.startsWith('//') || /[\\\u0000-\u0020]/.test(decoded)) {
    return fallback
  }
  const pathname = new URL(decoded, 'https://application.invalid').pathname.replace(/\/+$/, '')
  return pathname === '/login' ? fallback : value
}

// 数据模式只影响业务数据，会话始终要求后端访问令牌。
export function sessionIsActive(_mode, session) {
  if (session.signedOut === true) {
    return false
  }
  return (
    session.mode !== 'demo' &&
    typeof session.accessToken === 'string' &&
    session.accessToken.trim().length > 0
  )
}

export function sameSessionIdentity(previous, next) {
  return ['mode', 'signedOut', 'accessToken', 'tenantId', 'userId'].every(
    (key) => previous[key] === next[key]
  )
}
