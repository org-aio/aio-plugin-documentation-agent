import type { TableColumnCtx } from 'element-plus'

type DateValue = Date | string | number | null | undefined

const pad = (value: number) => String(value).padStart(2, '0')

export const formatDate = (value: DateValue, format = 'YYYY-MM-DD HH:mm:ss'): string => {
  if (value === null || value === undefined || value === '') {
    return ''
  }
  const date = value instanceof Date ? value : new Date(value)
  if (Number.isNaN(date.getTime())) {
    return String(value)
  }
  const tokens: Record<string, string> = {
    YYYY: String(date.getFullYear()),
    MM: pad(date.getMonth() + 1),
    DD: pad(date.getDate()),
    HH: pad(date.getHours()),
    mm: pad(date.getMinutes()),
    ss: pad(date.getSeconds())
  }
  return format.replace(/YYYY|MM|DD|HH|mm|ss/g, (token) => tokens[token])
}

export const dateFormatter = (
  _row: unknown,
  _column: TableColumnCtx<Recordable>,
  cellValue: DateValue
): string => formatDate(cellValue)

export const dateFormatter2 = (
  _row: unknown,
  _column: TableColumnCtx<Recordable>,
  cellValue: DateValue
): string => formatDate(cellValue, 'YYYY-MM-DD')
