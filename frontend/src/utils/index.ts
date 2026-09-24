export const fileSizeFormatter = (
  _row: unknown,
  _column: unknown,
  value: number | string | null | undefined
): string => {
  const bytes = Number(value)
  if (!Number.isFinite(bytes) || bytes < 0) {
    return '-'
  }
  if (bytes < 1024) {
    return `${bytes} B`
  }
  const units = ['KB', 'MB', 'GB', 'TB']
  let size = bytes / 1024
  let unit = units[0]
  for (let index = 1; index < units.length && size >= 1024; index += 1) {
    size /= 1024
    unit = units[index]
  }
  return `${size.toFixed(size >= 10 ? 1 : 2)} ${unit}`
}
