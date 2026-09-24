import assert from 'node:assert/strict'
import test from 'node:test'
import { buildAvatarUploadForm, resolveApiAssetUrl } from './profileUpload.mjs'

test('相对头像地址保留 API 根路径，绝对地址和内嵌预览不变', () => {
  assert.equal(
    resolveApiAssetUrl('/admin-api/', '/infra/file/content/42'),
    '/admin-api/infra/file/content/42'
  )
  assert.equal(
    resolveApiAssetUrl('https://example.invalid/admin-api', 'infra/file/content/42'),
    'https://example.invalid/admin-api/infra/file/content/42'
  )
  assert.equal(
    resolveApiAssetUrl('/admin-api', 'https://cdn.example.invalid/avatar.png'),
    'https://cdn.example.invalid/avatar.png'
  )
  assert.equal(
    resolveApiAssetUrl('/admin-api', 'data:image/png;base64,avatar'),
    'data:image/png;base64,avatar'
  )
})

test('头像上传表单提交原始文件和头像目录', async () => {
  const file = new File(['avatar'], 'me.png', { type: 'image/png' })
  const data = buildAvatarUploadForm(file)
  const uploaded = data.get('file')
  assert.ok(uploaded instanceof File)
  assert.equal(uploaded.name, file.name)
  assert.equal(uploaded.type, file.type)
  assert.equal(await uploaded.text(), 'avatar')
  assert.equal(data.get('directory'), 'avatar')
})
