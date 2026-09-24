import CryptoJS from 'crypto-js'

export const CAPTCHA_WIDTH = 310
export const CAPTCHA_HEIGHT = 155
export const CAPTCHA_LIFETIME = 120_000
const types = new Set(['blockPuzzle', 'clickWord'])

const failure = (messageKey, message) =>
  Object.assign(new Error(message || messageKey), { messageKey, serverMessage: message })
const cancelled = () => new DOMException('验证码请求已取消', 'AbortError')

export function parseCaptchaConfig(value) {
  if (!value || typeof value.enabled !== 'boolean' || !types.has(value.type)) {
    throw failure('account.captchaConfigInvalid')
  }
  return { enabled: value.enabled, type: value.type }
}

function checkedResponse(response) {
  if (!response || typeof response.repCode !== 'string') {
    throw failure('account.captchaResponseInvalid')
  }
  if (response.repCode !== '0000') {
    const message = typeof response.repMsg === 'string' ? response.repMsg : undefined
    throw failure('account.captchaFailed', message)
  }
  return response.repData
}

export function scaleCaptchaPoint(x, y, width, height) {
  if (![x, y, width, height].every(Number.isFinite) || width <= 0 || height <= 0) {
    throw failure('account.captchaResponseInvalid')
  }
  return {
    x: Math.max(0, Math.min(CAPTCHA_WIDTH, (x * CAPTCHA_WIDTH) / width)),
    y: Math.max(0, Math.min(CAPTCHA_HEIGHT, (y * CAPTCHA_HEIGHT) / height))
  }
}

// 复用原验证码组件的 Anji 协议；同一份坐标明文用于检查和登录证明。
function encrypt(value, secretKey) {
  if (!secretKey) {
    return value
  }
  const key = CryptoJS.enc.Utf8.parse(secretKey)
  return CryptoJS.AES.encrypt(CryptoJS.enc.Utf8.parse(value), key, {
    mode: CryptoJS.mode.ECB,
    padding: CryptoJS.pad.Pkcs7
  }).toString()
}

export function createCaptchaSession({
  type,
  post,
  now = Date.now,
  clientUid = globalThis.crypto?.randomUUID?.()
}) {
  if (!types.has(type)) {
    throw failure('account.captchaConfigInvalid')
  }
  let revision = 0
  let controller
  let challenge

  const cancel = () => {
    revision += 1
    controller?.abort()
    controller = undefined
    challenge = undefined
  }

  const load = async () => {
    cancel()
    const requestRevision = revision
    controller = new AbortController()
    const response = await post(
      '/system/captcha/get',
      { captchaType: type, clientUid },
      controller.signal
    )
    if (requestRevision !== revision) {
      throw cancelled()
    }
    const data = checkedResponse(response)
    if (
      !data ||
      typeof data.token !== 'string' ||
      !data.token ||
      typeof data.originalImageBase64 !== 'string' ||
      !data.originalImageBase64 ||
      (data.secretKey != null && typeof data.secretKey !== 'string') ||
      (type === 'blockPuzzle' &&
        (typeof data.jigsawImageBase64 !== 'string' || !data.jigsawImageBase64)) ||
      (type === 'clickWord' &&
        (!Array.isArray(data.wordList) ||
          !data.wordList.length ||
          data.wordList.some((word) => typeof word !== 'string' || !word)))
    ) {
      throw failure('account.captchaResponseInvalid')
    }
    challenge = { ...data, type, expiresAt: now() + CAPTCHA_LIFETIME }
    return challenge
  }

  const verify = async (points) => {
    const current = challenge
    if (!current || now() >= current.expiresAt) {
      challenge = undefined
      throw failure('account.captchaExpired')
    }
    const selected = Array.isArray(points) ? points : [points]
    const expectedCount = type === 'clickWord' ? current.wordList.length : 1
    if (
      selected.length !== expectedCount ||
      selected.some(
        (point) =>
          !point ||
          !Number.isFinite(point.x) ||
          !Number.isFinite(point.y) ||
          point.x < 0 ||
          point.x > CAPTCHA_WIDTH ||
          point.y < 0 ||
          point.y > CAPTCHA_HEIGHT
      ) ||
      (type === 'blockPuzzle' && (Array.isArray(points) || points.y !== 5)) ||
      (type === 'clickWord' && !Array.isArray(points))
    ) {
      throw failure('account.captchaIncomplete')
    }
    // 服务端每次检查都会消耗挑战；失败、超时也必须重新获取。
    challenge = undefined
    const requestRevision = revision
    const pointText = JSON.stringify(points)
    const response = await post(
      '/system/captcha/check',
      {
        captchaType: type,
        pointJson: encrypt(pointText, current.secretKey),
        token: current.token
      },
      controller.signal
    )
    if (requestRevision !== revision) {
      throw cancelled()
    }
    checkedResponse(response)
    return encrypt(`${current.token}---${pointText}`, current.secretKey)
  }

  return { load, verify, cancel }
}
