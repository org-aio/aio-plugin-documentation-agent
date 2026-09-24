import assert from 'node:assert/strict'
import test from 'node:test'
import {
  buildCategoryTree,
  createSeedState,
  deleteCategory,
  deleteRecord,
  ExampleDataError,
  queryRecords,
  readState,
  saveCategory,
  saveRecord,
  writeState
} from './model.mjs'

function memoryStorage() {
  const entries = new Map()
  return {
    getItem: (key) => entries.get(key) ?? null,
    setItem: (key, value) => entries.set(key, value)
  }
}

test('分类选择包含子分类，搜索、状态与分页同时生效', () => {
  const state = createSeedState()
  const office = queryRecords(state, { categoryId: 'office', pageSize: 2, page: 99 })
  assert.equal(office.total, 6)
  assert.equal(office.page, 3)
  assert.equal(office.items.length, 2)
  const filtered = queryRecords(state, {
    categoryId: 'office',
    keyword: ' off-005 ',
    status: 'disabled'
  })
  assert.deepEqual(
    filtered.items.map((record) => record.name),
    ['订书机']
  )
  assert.equal(queryRecords(state, { keyword: '不存在' }).page, 1)
  assert.equal(buildCategoryTree(state).find((node) => node.id === 'office').recordCount, 6)
})

test('新增、编辑和删除保留数据不变量，编码忽略大小写判重', () => {
  const initial = createSeedState()
  const input = { name: ' 测试记录 ', code: ' DEMO-100 ', categoryId: 'paper', status: 'enabled' }
  const created = saveRecord(initial, input, null, new Date('2026-02-01T00:00:00Z'))
  const record = created.records[0]
  assert.equal(record.name, '测试记录')
  assert.equal(record.code, 'DEMO-100')
  assert.equal(initial.records.length, 16)
  assert.equal(created.records.length, 17)
  assert.throws(() => saveRecord(created, { ...input, code: 'demo-100' }), /codeDuplicate/)
  assert.throws(() => saveRecord(created, { ...input, code: '含空 格' }), /codeFormat/)
  assert.throws(
    () => saveRecord(created, { ...input, code: 'DEMO-101', categoryId: 'missing' }),
    /categoryRequired/
  )
  const updated = saveRecord(
    created,
    { ...input, status: 'disabled' },
    record.id,
    new Date('2026-02-02T00:00:00Z')
  )
  assert.equal(updated.records.find((item) => item.id === record.id).status, 'disabled')
  assert.equal(
    updated.records.find((item) => item.id === record.id).updatedAt,
    '2026-02-02T00:00:00.000Z'
  )
  assert.equal(deleteRecord(updated, record.id).records.length, initial.records.length)
})

test('分类有子节点或关联记录时不可删除，空分类可重命名和删除', () => {
  const state = createSeedState()
  assert.throws(() => deleteCategory(state, 'office'), /categoryHasChildren/)
  assert.throws(() => deleteCategory(state, 'paper'), /categoryHasRecords/)
  const created = saveCategory(state, { name: ' 临时分类 ', parentId: 'office' })
  const category = created.categories.at(-1)
  assert.equal(category.name, '临时分类')
  assert.throws(
    () => saveCategory(created, { name: '临时分类', parentId: 'office' }),
    /categoryDuplicate/
  )
  const renamed = saveCategory(created, { name: '空分类', parentId: 'office' }, category.id)
  assert.equal(renamed.categories.at(-1).name, '空分类')
  assert.throws(
    () => saveCategory(renamed, { name: '空分类', parentId: null }, category.id),
    /categoryMove/
  )
  assert.equal(deleteCategory(renamed, category.id).categories.length, state.categories.length)
})

test('本地存储往返保留更改且按应用键隔离', () => {
  const storage = memoryStorage()
  const state = deleteRecord(createSeedState(), 'record-1')
  writeState(storage, 'app-one:examples:v1', state)
  assert.deepEqual(readState(storage, 'app-one:examples:v1'), state)
  assert.equal(readState(storage, 'app-two:examples:v1').records.length, 16)
  const failure = new Error('storage quota exceeded')
  assert.throws(
    () =>
      writeState(
        {
          ...storage,
          setItem: () => {
            throw failure
          }
        },
        'app-one',
        state
      ),
    failure
  )
})

test('损坏或关联不完整的缓存不会作为有效状态载入', () => {
  const storage = memoryStorage()
  storage.setItem('demo', '{')
  assert.throws(() => readState(storage, 'demo'), /storageInvalid/)
  const cyclic = createSeedState()
  cyclic.categories[0].parentId = 'stationery'
  storage.setItem('demo', JSON.stringify(cyclic))
  assert.throws(() => readState(storage, 'demo'), /storageInvalid/)
  const missingCategory = createSeedState()
  missingCategory.records[0].categoryId = 'missing'
  storage.setItem('demo', JSON.stringify(missingCategory))
  assert.throws(() => readState(storage, 'demo'), /storageInvalid/)
  const duplicate = createSeedState()
  duplicate.records[1].code = duplicate.records[0].code.toLowerCase()
  storage.setItem('demo', JSON.stringify(duplicate))
  assert.throws(() => readState(storage, 'demo'), /storageInvalid/)
})

test('多级祖先缺失和同级分类重名返回明确的缓存格式错误', () => {
  const storage = memoryStorage()
  const missingAncestor = createSeedState()
  missingAncestor.categories[0].parentId = 'stationery'
  missingAncestor.categories[1].parentId = 'missing'
  storage.setItem('demo', JSON.stringify(missingAncestor))
  assert.throws(
    () => readState(storage, 'demo'),
    (error) => {
      assert.ok(error instanceof ExampleDataError)
      assert.equal(error.key, 'examples.errors.storageInvalid')
      return true
    }
  )
  const duplicateSiblings = createSeedState()
  duplicateSiblings.categories[2].name = '文具'
  storage.setItem('demo', JSON.stringify(duplicateSiblings))
  assert.throws(() => readState(storage, 'demo'), /storageInvalid/)
})
