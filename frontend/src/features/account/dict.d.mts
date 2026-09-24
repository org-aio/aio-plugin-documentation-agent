export interface DictOption {
  dictType: string
  label: string
  value: unknown
  colorType: string
  cssClass: string
}
export function parseDictDataList(value: unknown): Map<string, DictOption[]>
