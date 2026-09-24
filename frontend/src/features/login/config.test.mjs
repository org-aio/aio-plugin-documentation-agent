import assert from 'node:assert/strict'
import test from 'node:test'
import {
  DEFAULT_LOGIN_CONFIGURATION,
  normalizeLoginConfiguration,
  validateLoginConfiguration
} from './config.mjs'

test('old and damaged login preferences restore field defaults without losing valid customizations', () => {
  assert.deepEqual(normalizeLoginConfiguration(undefined), DEFAULT_LOGIN_CONFIGURATION)
  const application = { ...DEFAULT_LOGIN_CONFIGURATION, formTitle: '应用登录', showTenant: true }
  const result = normalizeLoginConfiguration(
    {
      layout: 'centered',
      formTitle: null,
      showTenant: 'false',
      backgroundOverlay: Infinity,
      heroImage: 'javascript:alert(1)'
    },
    application
  )
  assert.equal(result.layout, 'centered')
  assert.equal(result.formTitle, '应用登录')
  assert.equal(result.showTenant, true)
  assert.equal(result.heroImage, '')
  assert.equal(result.backgroundOverlay, 0.25)
  assert.deepEqual(normalizeLoginConfiguration([]), DEFAULT_LOGIN_CONFIGURATION)
})

test('configuration rejects invalid fields, unsupported schemes and client security switches', () => {
  for (const input of [
    null,
    [],
    { captchaEnabled: false },
    { showHero: 1 },
    { backgroundColor: 'red' },
    { backgroundImage: '//external.invalid/a.png' },
    { heroImage: 'https://name:pass@example.invalid/logo.png' },
    { qrcodeImage: 'javascript:alert(1)' },
    { qrcodeLink: '//external.invalid/qrcode' }
  ]) {
    assert.throws(() => validateLoginConfiguration(input), /login configuration/)
  }
  assert.equal(
    validateLoginConfiguration({ backgroundImage: 'https://example.invalid/a.png' })
      .backgroundImage,
    'https://example.invalid/a.png'
  )
  assert.equal(
    validateLoginConfiguration({ qrcodeLink: 'https://example.invalid/qrcode' }).qrcodeLink,
    'https://example.invalid/qrcode'
  )
})
