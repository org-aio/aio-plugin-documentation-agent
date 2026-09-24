import assert from 'node:assert/strict'
import { createDecipheriv } from 'node:crypto'
import test from 'node:test'
import {
  CAPTCHA_LIFETIME,
  createCaptchaSession,
  parseCaptchaConfig,
  scaleCaptchaPoint
} from './captcha.mjs'

const key = '1234567890abcdef'
const challenge = (token = 'challenge-token', secretKey = key) => ({
  repCode: '0000',
  repData: {
    token,
    secretKey,
    originalImageBase64: 'background-image',
    jigsawImageBase64: 'puzzle-image',
    wordList: ['云', '山', '水']
  }
})
const decrypt = (value) => {
  const decipher = createDecipheriv('aes-128-ecb', Buffer.from(key), null)
  return Buffer.concat([decipher.update(Buffer.from(value, 'base64')), decipher.final()]).toString()
}
const pending = () => {
  let resolve
  const promise = new Promise((complete) => {
    resolve = complete
  })
  return { promise, resolve }
}

test('验证码配置由后端决定；未知、缺失或伪造配置不得当作关闭验证码', () => {
  assert.deepEqual(parseCaptchaConfig({ enabled: false, type: 'clickWord' }), {
    enabled: false,
    type: 'clickWord'
  })
  for (const value of [
    undefined,
    {},
    { enabled: 'false', type: 'blockPuzzle' },
    { enabled: true, type: 'unknown' }
  ]) {
    assert.throws(() => parseCaptchaConfig(value), { messageKey: 'account.captchaConfigInvalid' })
  }
})

test('手机缩放坐标按 Anji 原图 310×155 转换，边界坐标保持在图内', () => {
  assert.deepEqual(scaleCaptchaPoint(75, 37.5, 150, 75), { x: 155, y: 77.5 })
  assert.deepEqual(scaleCaptchaPoint(-5, 90, 150, 75), { x: 0, y: 155 })
  assert.throws(() => scaleCaptchaPoint(1, 1, 0, 0))
})

test('滑块检查与登录证明使用同一明文，并兼容原 AES-ECB/PKCS7', async () => {
  const points = { x: 126.5, y: 5 }
  const pointText = JSON.stringify(points)
  const calls = []
  const session = createCaptchaSession({
    type: 'blockPuzzle',
    clientUid: 'client',
    post: async (url, data) => {
      calls.push(url)
      if (url.endsWith('/get')) {
        assert.deepEqual(data, { captchaType: 'blockPuzzle', clientUid: 'client' })
        return challenge()
      }
      assert.equal(data.token, 'challenge-token')
      assert.equal(data.captchaType, 'blockPuzzle')
      assert.equal(decrypt(data.pointJson), pointText)
      return { repCode: '0000' }
    }
  })
  await session.load()
  assert.equal(decrypt(await session.verify(points)), `challenge-token---${pointText}`)
  await assert.rejects(session.verify(points), { messageKey: 'account.captchaExpired' })
  assert.deepEqual(calls, ['/system/captcha/get', '/system/captcha/check'])
})

test('无密钥点选模式保留原明文协议，只在所有文字选完后提交', async () => {
  let checks = 0
  const points = [
    { x: 12, y: 20 },
    { x: 120, y: 60 },
    { x: 260, y: 130 }
  ]
  const session = createCaptchaSession({
    type: 'clickWord',
    post: async (url, data) => {
      if (url.endsWith('/get')) {
        return challenge('word-token', '')
      }
      checks += 1
      assert.equal(data.pointJson, JSON.stringify(points))
      return { repCode: '0000' }
    }
  })
  await session.load()
  await assert.rejects(session.verify(points.slice(0, 1)), {
    messageKey: 'account.captchaIncomplete'
  })
  assert.equal(checks, 0)
  assert.equal(await session.verify(points), `word-token---${JSON.stringify(points)}`)
})

test('服务器拒绝和伪成功响应都不产生证明，失败挑战不能复用', async () => {
  for (const response of [
    { repCode: '6111', repMsg: '验证失败' },
    { code: 0, data: true },
    { repCode: 0 }
  ]) {
    let checks = 0
    const session = createCaptchaSession({
      type: 'blockPuzzle',
      post: async (url) => {
        if (url.endsWith('/get')) {
          return challenge()
        }
        checks += 1
        return response
      }
    })
    await session.load()
    await assert.rejects(session.verify({ x: 80, y: 5 }))
    await assert.rejects(session.verify({ x: 80, y: 5 }))
    assert.equal(checks, 1)
  }
})

test('网络失败也消耗本地挑战，不产生可复用证明', async () => {
  const session = createCaptchaSession({
    type: 'blockPuzzle',
    post: async (url) => {
      if (url.endsWith('/get')) {
        return challenge()
      }
      throw new Error('offline')
    }
  })
  await session.load()
  await assert.rejects(session.verify({ x: 80, y: 5 }), /offline/)
  await assert.rejects(session.verify({ x: 80, y: 5 }), { messageKey: 'account.captchaExpired' })
})

test('失效或不完整的图片响应不能建立可验证挑战', async () => {
  for (const response of [
    { repCode: '6110' },
    { repCode: '0000' },
    { repCode: '0000', repData: { token: 'token' } }
  ]) {
    const session = createCaptchaSession({ type: 'blockPuzzle', post: async () => response })
    await assert.rejects(session.load())
    await assert.rejects(session.verify({ x: 80, y: 5 }))
  }
})

test('刷新后旧图片响应即使忽略 AbortSignal 也不能覆盖新挑战', async () => {
  const oldRequest = pending()
  let gets = 0
  let oldSignal
  const session = createCaptchaSession({
    type: 'blockPuzzle',
    post: async (url, data, signal) => {
      if (url.endsWith('/check')) {
        assert.equal(data.token, 'fresh-token')
        return { repCode: '0000' }
      }
      gets += 1
      if (gets === 1) {
        oldSignal = signal
        return oldRequest.promise
      }
      return challenge('fresh-token')
    }
  })
  const oldLoad = session.load()
  const rejected = assert.rejects(oldLoad, { name: 'AbortError' })
  const fresh = await session.load()
  assert.equal(fresh.token, 'fresh-token')
  assert.equal(oldSignal.aborted, true)
  oldRequest.resolve(challenge('stale-token'))
  await rejected
  assert.match(decrypt(await session.verify({ x: 80, y: 5 })), /^fresh-token---/)
})

test('过期挑战不再请求检查；取消后到达的成功响应不能产生登录证明', async () => {
  let time = 0
  const check = pending()
  let checks = 0
  let checkSignal
  const session = createCaptchaSession({
    type: 'blockPuzzle',
    now: () => time,
    post: async (url, _data, signal) => {
      if (url.endsWith('/get')) {
        return challenge()
      }
      checks += 1
      checkSignal = signal
      return check.promise
    }
  })
  await session.load()
  time = CAPTCHA_LIFETIME
  await assert.rejects(session.verify({ x: 80, y: 5 }), { messageKey: 'account.captchaExpired' })
  assert.equal(checks, 0)
  await session.load()
  const checking = session.verify({ x: 80, y: 5 })
  const rejected = assert.rejects(checking, { name: 'AbortError' })
  session.cancel()
  assert.equal(checkSignal.aborted, true)
  check.resolve({ repCode: '0000' })
  await rejected
})
