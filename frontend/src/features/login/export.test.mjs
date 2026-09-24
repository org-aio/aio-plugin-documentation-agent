import assert from 'node:assert/strict'
import test from 'node:test'
import { normalizeLoginConfiguration, validateLoginConfiguration } from './config.mjs'
import { exportLoginConfiguration, LoginExportError } from './export.mjs'

const svg = '<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"></svg>'
const svgUrl = `data:image/svg+xml;base64,${Buffer.from(svg).toString('base64')}`
const options = {
  pageUrl: 'https://admin.example.test/console/login',
  resolveAssetUrl: (value) => `/console/${value.replace(/^\/+/, '')}`
}

test('export embeds generated public images using the deployment prefix and preserves source settings', async () => {
  const input = normalizeLoginConfiguration({
    backgroundImage: '/login-background.svg',
    heroImage: 'login-hero.svg',
    brandImage: '/brand.svg',
    qrcodeImage: '/qrcode.svg',
    layout: 'centered',
    formTitle: '专属登录'
  })
  const calls = []
  const result = await exportLoginConfiguration(input, {
    ...options,
    fetchAsset: async (url, init) => {
      calls.push({ url, init })
      return new Response(svg, { headers: { 'content-type': 'image/svg+xml; charset=utf-8' } })
    }
  })
  assert.deepEqual(
    calls.map(({ url }) => url),
    [
      'https://admin.example.test/console/login-background.svg',
      'https://admin.example.test/console/login-hero.svg',
      'https://admin.example.test/console/brand.svg',
      'https://admin.example.test/console/qrcode.svg'
    ]
  )
  assert.ok(
    calls.every(({ init }) => init.credentials === 'same-origin' && init.redirect === 'error')
  )
  assert.equal(result.backgroundImage, svgUrl)
  assert.equal(result.heroImage, svgUrl)
  assert.equal(result.brandImage, svgUrl)
  assert.equal(result.qrcodeImage, svgUrl)
  assert.equal(result.formTitle, '专属登录')
  assert.equal(result.layout, 'centered')
  assert.equal(input.backgroundImage, '/login-background.svg')
  assert.equal(input.heroImage, 'login-hero.svg')
  assert.equal(input.brandImage, '/brand.svg')
  assert.equal(input.qrcodeImage, '/qrcode.svg')
})

test('remote URLs and existing embedded images are preserved without network access', async () => {
  const result = await exportLoginConfiguration(
    {
      backgroundImage: 'https://images.example.test/background.png',
      heroImage: svgUrl,
      brandImage: '',
      qrcodeImage: ''
    },
    {
      ...options,
      fetchAsset: () => assert.fail('Remote and embedded images must not be fetched')
    }
  )
  assert.equal(result.backgroundImage, 'https://images.example.test/background.png')
  assert.equal(result.heroImage, svgUrl)
})

test('export accepts valid CLI assets above the 1 MB upload limit and infers generic content types', async () => {
  const content = `<svg xmlns="http://www.w3.org/2000/svg"><!--${'x'.repeat(1024 * 1024)}--></svg>`
  const result = await exportLoginConfiguration(
    { backgroundImage: '/login-background.svg' },
    {
      ...options,
      fetchAsset: async () =>
        new Response(content, { headers: { 'content-type': 'application/octet-stream' } })
    }
  )
  assert.equal(
    result.backgroundImage,
    `data:image/svg+xml;base64,${Buffer.from(content).toString('base64')}`
  )
})

test('failed, missing, unsupported, empty and oversized images reject the entire export with field context', async (context) => {
  const failures = {
    network: async () => {
      throw new Error('offline')
    },
    missing: async () => new Response('missing', { status: 404 }),
    htmlFallback: async () =>
      new Response('<html>SPA fallback</html>', { headers: { 'content-type': 'text/html' } }),
    empty: async () => new Response('', { headers: { 'content-type': 'image/png' } }),
    oversized: async () =>
      new Response(new Uint8Array(5 * 1024 * 1024 + 1), {
        headers: { 'content-type': 'image/png' }
      })
  }
  for (const [name, fetchAsset] of Object.entries(failures)) {
    await context.test(name, async () => {
      const input = { backgroundImage: '/login-background.svg', heroImage: svgUrl }
      await assert.rejects(exportLoginConfiguration(input, { ...options, fetchAsset }), (error) => {
        assert.ok(error instanceof LoginExportError)
        assert.equal(error.field, 'backgroundImage')
        assert.ok(error.cause instanceof Error)
        return true
      })
      assert.deepEqual(input, { backgroundImage: '/login-background.svg', heroImage: svgUrl })
    })
  }
})

test('a local source resolving off origin is rejected before any request', async () => {
  await assert.rejects(
    exportLoginConfiguration(
      { heroImage: '/hero.svg' },
      {
        ...options,
        resolveAssetUrl: () => 'https://other.example.test/hero.svg',
        fetchAsset: () => assert.fail('Cross-origin local assets must not be fetched')
      }
    ),
    (error) => error instanceof LoginExportError && error.field === 'heroImage'
  )
})

test('serialized export remains a flat valid presentation configuration with no account or security state', async () => {
  const result = await exportLoginConfiguration(
    {
      layout: 'split',
      showTenant: true,
      backgroundImage: '/background.svg',
      username: 'private-user',
      password: 'private-password',
      captchaEnabled: false,
      login: { formTitle: 'unexpected-wrapper' }
    },
    {
      ...options,
      fetchAsset: async () => new Response(svg, { headers: { 'content-type': 'image/svg+xml' } })
    }
  )
  const restored = JSON.parse(JSON.stringify(result))
  assert.deepEqual(validateLoginConfiguration(restored), result)
  assert.equal(restored.backgroundImage, svgUrl)
  assert.equal(restored.showTenant, true)
  for (const field of ['username', 'password', 'captchaEnabled', 'login']) {
    assert.equal(Object.hasOwn(restored, field), false)
  }
})
