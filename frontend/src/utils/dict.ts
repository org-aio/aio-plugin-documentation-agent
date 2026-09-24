import { reactive } from 'vue'
import { DICTIONARIES } from '@/features/system-demo/seeds.mjs'
import { parseDictDataList } from '@/features/account/dict.mjs'

export type DictColor = '' | 'primary' | 'success' | 'warning' | 'info' | 'danger'

export interface DictDataType {
  dictType: string
  label: string
  value: string | number | boolean
  colorType: DictColor
  cssClass: string
}

export interface NumberDictDataType extends DictDataType {
  value: number
}

export interface StringDictDataType extends DictDataType {
  value: string
}

export enum DICT_TYPE {
  COMMON_STATUS = 'common_status',
  USER_TYPE = 'user_type',
  SYSTEM_USER_SEX = 'system_user_sex',
  SYSTEM_MENU_TYPE = 'system_menu_type',
  SYSTEM_ROLE_TYPE = 'system_role_type',
  SYSTEM_DATA_SCOPE = 'system_data_scope',
  SYSTEM_LOGIN_TYPE = 'system_login_type',
  SYSTEM_LOGIN_RESULT = 'system_login_result',
  INFRA_BOOLEAN_STRING = 'infra_boolean_string',
  INFRA_CONFIG_TYPE = 'infra_config_type',
  INFRA_FILE_STORAGE = 'infra_file_storage',
  INFRA_OPERATE_TYPE = 'infra_operate_type',
  INFRA_API_ERROR_LOG_PROCESS_STATUS = 'infra_api_error_log_process_status'
}

const dictionaries = reactive<Record<string, DictDataType[]>>({})

for (const [dictType, options] of Object.entries(DICTIONARIES)) {
  dictionaries[dictType] = options.map(([label, value, colorType]) => ({
    dictType,
    label,
    value,
    colorType: colorType as DictColor,
    cssClass: ''
  }))
}

// 宿主加载真实字典后可以覆盖枚举默认文案。
export const setDictOptions = (dictType: string, options: DictDataType[]) => {
  dictionaries[dictType] = options
}

// API 模式用后端字典覆盖演示枚举；同一字典类型整体替换，避免新旧值混用。
export const setDictMap = (list: unknown): void => {
  for (const [dictType, options] of parseDictDataList(list)) {
    dictionaries[dictType] = options as DictDataType[]
  }
}

export const getDictOptions = (dictType: string): DictDataType[] => dictionaries[dictType] ?? []

export const getIntDictOptions = (dictType: string): NumberDictDataType[] =>
  getDictOptions(dictType).map((option) => ({ ...option, value: Number(option.value) }))

export const getStrDictOptions = (dictType: string): StringDictDataType[] =>
  getDictOptions(dictType).map((option) => ({ ...option, value: String(option.value) }))

export const getBoolDictOptions = (dictType: string) =>
  getDictOptions(dictType).map((option) => ({ ...option, value: String(option.value) === 'true' }))

export const getDictObj = (dictType: string, value: unknown): DictDataType | undefined =>
  getDictOptions(dictType).find((option) => String(option.value) === String(value))

export const getDictLabel = (dictType: string, value: unknown): string =>
  getDictObj(dictType, value)?.label ?? String(value ?? '')
