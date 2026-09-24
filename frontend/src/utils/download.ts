const save = (data: BlobPart, fileName: string, type: string) => {
  const url = URL.createObjectURL(new Blob([data], { type }))
  const link = document.createElement('a')
  link.href = url
  link.download = fileName
  link.click()
  URL.revokeObjectURL(url)
}

export default {
  excel: (data: BlobPart, fileName: string) => save(data, fileName, 'application/vnd.ms-excel'),
  word: (data: BlobPart, fileName: string) => save(data, fileName, 'application/msword'),
  zip: (data: BlobPart, fileName: string) => save(data, fileName, 'application/zip'),
  html: (data: BlobPart, fileName: string) => save(data, fileName, 'text/html'),
  markdown: (data: BlobPart, fileName: string) => save(data, fileName, 'text/markdown'),
  json: (data: BlobPart, fileName: string) => save(data, fileName, 'application/json')
}
