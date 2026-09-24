export const logoFileAccept = '.svg,.png,.jpg,.jpeg,.webp,.ico'
const maxLogoBytes = 1024 * 1024
const fileTypes: Record<string, string> = {
  svg: 'image/svg+xml',
  png: 'image/png',
  jpg: 'image/jpeg',
  jpeg: 'image/jpeg',
  webp: 'image/webp',
  ico: 'image/x-icon'
}
const allowedTypes = new Set([...Object.values(fileTypes), 'image/vnd.microsoft.icon'])

export class LogoFileError extends Error {
  readonly messageKey: string

  constructor(messageKey: string) {
    super(messageKey)
    this.name = 'LogoFileError'
    this.messageKey = messageKey
  }
}

const readDataUrl = (file: Blob): Promise<string> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = () => {
      if (typeof reader.result !== 'string') {
        reject(new LogoFileError('settings.logoReadFailed'))
        return
      }
      resolve(reader.result)
    }
    reader.onerror = () => reject(new LogoFileError('settings.logoReadFailed'))
    reader.onabort = () => reject(new LogoFileError('settings.logoReadFailed'))
    reader.readAsDataURL(file)
  })
}

const validateImage = (dataUrl: string): Promise<void> => {
  return new Promise((resolve, reject) => {
    const image = new Image()
    const timeout = window.setTimeout(() => {
      finish(new LogoFileError('settings.logoDecodeFailed'))
    }, 15000)
    const finish = (error?: LogoFileError) => {
      window.clearTimeout(timeout)
      image.onload = null
      image.onerror = null
      if (error) {
        reject(error)
        return
      }
      resolve()
    }
    image.onerror = () => finish(new LogoFileError('settings.logoDecodeFailed'))
    image.onload = () => {
      if (!image.naturalWidth || !image.naturalHeight) {
        finish(new LogoFileError('settings.logoDecodeFailed'))
        return
      }
      finish()
    }
    image.src = dataUrl
  })
}

export const readLogoFile = async (file: File): Promise<string> => {
  const extension = file.name.split('.').pop()?.toLowerCase() || ''
  const mimeType = file.type.toLowerCase() || fileTypes[extension]
  if (!fileTypes[extension] || !allowedTypes.has(mimeType)) {
    throw new LogoFileError('settings.logoUnsupported')
  }
  if (file.size > maxLogoBytes) {
    throw new LogoFileError('settings.logoTooLarge')
  }
  if (!file.size) {
    throw new LogoFileError('settings.logoDecodeFailed')
  }
  // 内嵌图片可随偏好保存；必须解码成功后，界面才替换当前图标。
  const imageFile = file.slice(0, file.size, mimeType)
  const dataUrl = await readDataUrl(imageFile)
  await validateImage(dataUrl)
  return dataUrl
}
