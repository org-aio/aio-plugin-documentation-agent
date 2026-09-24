import assert from 'node:assert/strict'
import test from 'node:test'
import { parseDictDataList } from './dict.mjs'

test('精简字典按 dictType 分组并保留颜色与样式', () => {
  const grouped = parseDictDataList([
    { dictType: 'common_status', label: '开启', value: 1, colorType: 'success', cssClass: '' },
    { dictType: 'common_status', label: '关闭', value: 0, colorType: 'danger', cssClass: 'x' },
    { dictType: 'user_type', label: '会员', value: 2, colorType: '', cssClass: '' }
  ])
  assert.deepEqual([...grouped.keys()].sort(), ['common_status', 'user_type'])
  assert.deepEqual(grouped.get('common_status'), [
    { dictType: 'common_status', label: '开启', value: 1, colorType: 'success', cssClass: '' },
    { dictType: 'common_status', label: '关闭', value: 0, colorType: 'danger', cssClass: 'x' }
  ])
})

test('非数组或缺失 dictType 的条目被忽略而不是抛错', () => {
  assert.equal(parseDictDataList(null).size, 0)
  assert.equal(parseDictDataList({}).size, 0)
  const grouped = parseDictDataList([
    null,
    { label: '无类型', value: 1 },
    { dictType: '   ', value: 2 },
    { dictType: 'ok', label: undefined, value: 'v', colorType: 5, cssClass: null }
  ])
  assert.deepEqual([...grouped.keys()], ['ok'])
  assert.deepEqual(grouped.get('ok'), [
    { dictType: 'ok', label: '', value: 'v', colorType: '', cssClass: '' }
  ])
})
