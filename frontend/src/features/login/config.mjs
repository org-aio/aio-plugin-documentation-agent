export const DEFAULT_LOGIN_CONFIGURATION = Object.freeze({
  layout: 'split',
  panelPosition: 'right',
  backgroundType: 'gradient',
  backgroundColor: '#0b1730',
  backgroundImage: '',
  backgroundOverlay: 0.25,
  heroTitle: '欢迎回来',
  heroDescription: '在这里，高效开启每一天的工作。',
  heroImage: '',
  formTitle: '账号登录',
  formDescription: '请输入您的账号和密码',
  brandImage: '/boxunlogo.png',
  showBrand: true,
  showHero: true,
  showTenant: false,
  rememberAccount: true,
  showThemeToggle: true,
  showFooter: false,
  footer: '',
  showQrcode: true,
  qrcodeImage: '/boxunqrcode.png',
  qrcodeLink:
    'https://mp.weixin.qq.com/mp/profile_ext?action=home&__biz=MzkyMzMzMjYzOA==&scene=124#wechat_redirect',
  qrcodeLabel: '关注我们',
  qrcodeDescription: ''
})

const choices = {
  layout: ['split', 'centered'],
  panelPosition: ['left', 'right'],
  backgroundType: ['gradient', 'solid', 'image']
}

const isImageSource = (value) => {
  if (!value) {
    return true
  }
  if (
    /^data:image\/(png|jpeg|webp|svg\+xml|x-icon|vnd.microsoft.icon);base64,[A-Za-z0-9+/=]+$/i.test(
      value
    )
  ) {
    return true
  }
  if (/^https?:\/\//i.test(value)) {
    try {
      const url = new URL(value)
      return !url.username && !url.password
    } catch {
      return false
    }
  }
  // eslint-disable-next-line no-control-regex -- 拒绝控制字符和 URL scheme。
  return !/^[a-z][a-z\d+.-]*:|^\/\/|[\x00-\x1f\\]/i.test(value)
}

const isSafeLink = (value) => {
  if (!value) {
    return true
  }
  if (!/^https?:\/\//i.test(value)) {
    return false
  }
  try {
    const url = new URL(value)
    return !url.username && !url.password
  } catch {
    return false
  }
}

const validField = (key, value) => {
  const fallback = DEFAULT_LOGIN_CONFIGURATION[key]
  if (typeof value !== typeof fallback) {
    return false
  }
  if (choices[key]) {
    return choices[key].includes(value)
  }
  if (key === 'backgroundOverlay') {
    return Number.isFinite(value) && value >= 0 && value <= 1
  }
  if (key === 'backgroundColor') {
    return /^#[\da-f]{6}$/i.test(value)
  }
  if (
    key === 'backgroundImage' ||
    key === 'heroImage' ||
    key === 'brandImage' ||
    key === 'qrcodeImage'
  ) {
    return isImageSource(value)
  }
  if (key === 'qrcodeLink') {
    return isSafeLink(value)
  }
  return (
    typeof value !== 'string' ||
    // eslint-disable-next-line no-control-regex -- 拒绝控制字符。
    (!/[\x00-\x08\x0b\x0c\x0e-\x1f]/.test(value) && value.length <= 2000)
  )
}

// 浏览器历史偏好逐项归一化，保留有效字段并恢复损坏字段的应用默认值。
export function normalizeLoginConfiguration(value, defaults = DEFAULT_LOGIN_CONFIGURATION) {
  const result = { ...DEFAULT_LOGIN_CONFIGURATION }
  for (const source of [defaults, value]) {
    if (!source || typeof source !== 'object' || Array.isArray(source)) {
      continue
    }
    for (const key of Object.keys(result)) {
      if (validField(key, source[key])) {
        result[key] = source[key]
      }
    }
  }
  return result
}

// CLI 必须显式报告错误，避免拼写错误或无效值静默进入生成结果。
export function validateLoginConfiguration(value) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    throw new Error('login configuration must be an object')
  }
  for (const [key, field] of Object.entries(value)) {
    if (!Object.hasOwn(DEFAULT_LOGIN_CONFIGURATION, key) || !validField(key, field)) {
      throw new Error(`Invalid login configuration field: ${key}`)
    }
  }
  return normalizeLoginConfiguration(value)
}
