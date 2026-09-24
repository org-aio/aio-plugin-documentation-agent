export function resolveApiAssetUrl(apiBase, value) {
  const asset = value.trim()
  if (!asset || /^(https?:\/\/|data:image\/|blob:)/i.test(asset)) {
    return asset
  }
  return `${apiBase.replace(/\/+$/, '')}/${asset.replace(/^\/+/, '')}`
}

export function buildAvatarUploadForm(file) {
  const data = new FormData()
  data.append('file', file, file.name || 'avatar.png')
  data.append('directory', 'avatar')
  return data
}
