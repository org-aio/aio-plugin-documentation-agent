// 把后端精简字典转换为显示层需要的结构；数据异常时返回空数组而不是抛错。
export function parseDictDataList(value) {
  const grouped = new Map()
  if (!Array.isArray(value)) {
    return grouped
  }
  for (const item of value) {
    if (!item || typeof item !== 'object') {
      continue
    }
    const dictType = typeof item.dictType === 'string' ? item.dictType.trim() : ''
    if (!dictType) {
      continue
    }
    const options = grouped.get(dictType) ?? []
    options.push({
      dictType,
      label: String(item.label ?? ''),
      value: item.value,
      colorType: typeof item.colorType === 'string' ? item.colorType : '',
      cssClass: typeof item.cssClass === 'string' ? item.cssClass : ''
    })
    grouped.set(dictType, options)
  }
  return grouped
}
