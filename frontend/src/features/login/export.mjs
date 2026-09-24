import { normalizeLoginConfiguration } from './config.mjs'

const imageFields = ['backgroundImage', 'heroImage', 'brandImage', 'qrcodeImage']
const maxImageBytes = 5 * 1024 * 1024
const imageTypes = {
  svg: 'image/svg+xml',
  png: 'image/png',
  jpg: 'image/jpeg',
  jpeg: 'image/jpeg',
  webp: 'image/webp',
  ico: 'image/x-icon'
}
const allowedTypes = new Set([...Object.values(imageTypes), 'image/vnd.microsoft.icon'])

export class LoginExportError extends Error {
  constructor(field, cause) {
    super(`Cannot export login image: ${field}`, { cause })
    this.name = 'LoginExportError'
    this.field = field
  }
}

const readImageDataUrl = async (blob, url) => {
  // 导出兼容 CLI 的 5 MB 资源上限，不受设置面板上传时的 1 MB 限制。
  if (!blob.size || blob.size > maxImageBytes) {
    throw new Error('Image must be non-empty and at most 5 MB')
  }
  const responseType = blob.type.split(';', 1)[0].toLowerCase()
  const extension = url.pathname.split('.').pop()?.toLowerCase()
  const type =
    !responseType || responseType === 'application/octet-stream'
      ? imageTypes[extension]
      : responseType
  if (!allowedTypes.has(type)) {
    throw new Error('Response is not a supported image')
  }
  const bytes = new Uint8Array(await blob.arrayBuffer())
  const chunks = []
  for (let offset = 0; offset < bytes.length; offset += 0x8000) {
    chunks.push(String.fromCharCode(...bytes.subarray(offset, offset + 0x8000)))
  }
  return `data:${type};base64,${btoa(chunks.join(''))}`
}

export async function exportLoginConfiguration(value, options) {
  const configuration = normalizeLoginConfiguration(value)
  const pageUrl = new URL(options.pageUrl)
  const fetchAsset = options.fetchAsset ?? globalThis.fetch
  await Promise.all(
    imageFields.map(async (field) => {
      const source = configuration[field]
      if (!source || /^(data:image\/|https?:\/\/)/i.test(source)) {
        return
      }
      try {
        const url = new URL(options.resolveAssetUrl(source), pageUrl)
        if (url.origin !== pageUrl.origin) {
          throw new Error('Local image must resolve to the application origin')
        }
        const response = await fetchAsset(url.href, {
          credentials: 'same-origin',
          redirect: 'error',
          signal: options.signal
        })
        if (!response.ok) {
          throw new Error(`Image request failed (${response.status})`)
        }
        configuration[field] = await readImageDataUrl(await response.blob(), url)
      } catch (error) {
        throw new LoginExportError(field, error)
      }
    })
  )
  return configuration
}
