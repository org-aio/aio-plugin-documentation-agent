const schemaVersion = 1

export class ExampleDataError extends Error {
  constructor(key) {
    super(key)
    this.name = 'ExampleDataError'
    this.key = key
  }
}

function requireValue(condition, key) {
  if (!condition) {
    throw new ExampleDataError(key)
  }
}

export function createSeedState() {
  const categories = [
    { id: 'office', parentId: null, name: '办公用品' },
    { id: 'stationery', parentId: 'office', name: '文具' },
    { id: 'paper', parentId: 'office', name: '纸张' },
    { id: 'electronics', parentId: null, name: '电子设备' },
    { id: 'accessories', parentId: 'electronics', name: '电脑配件' },
    { id: 'consumables', parentId: null, name: '日常耗材' }
  ]
  const samples = [
    ['中性笔', 'OFF-001', 'stationery'],
    ['文件夹', 'OFF-002', 'stationery'],
    ['便签纸', 'OFF-003', 'paper'],
    ['复印纸 A4', 'OFF-004', 'paper'],
    ['订书机', 'OFF-005', 'stationery'],
    ['桌面收纳盒', 'OFF-006', 'office'],
    ['无线键盘', 'ELE-001', 'accessories'],
    ['无线鼠标', 'ELE-002', 'accessories'],
    ['显示器', 'ELE-003', 'electronics'],
    ['扩展坞', 'ELE-004', 'accessories'],
    ['标签打印机', 'ELE-005', 'electronics'],
    ['网线', 'ELE-006', 'accessories'],
    ['清洁布', 'SUP-001', 'consumables'],
    ['垃圾袋', 'SUP-002', 'consumables'],
    ['纸杯', 'SUP-003', 'consumables'],
    ['包装胶带', 'SUP-004', 'consumables']
  ]
  const records = samples.map(([name, code, categoryId], index) => ({
    id: `record-${index + 1}`,
    name,
    code,
    categoryId,
    status: index % 5 === 4 ? 'disabled' : 'enabled',
    updatedAt: '2026-01-01T08:00:00.000Z'
  }))
  return { version: schemaVersion, categories, records }
}

export function categoryIdsWithDescendants(categories, categoryId) {
  const ids = new Set([categoryId])
  const pending = [categoryId]
  while (pending.length > 0) {
    const parentId = pending.pop()
    const children = categories.filter(
      (category) => category.parentId === parentId && !ids.has(category.id)
    )
    for (const child of children) {
      ids.add(child.id)
      pending.push(child.id)
    }
  }
  return ids
}

export function categoryOptions(categories) {
  const byId = new Map(categories.map((category) => [category.id, category]))
  return categories.map((category) => {
    const names = [category.name]
    const visited = new Set([category.id])
    let parentId = category.parentId
    while (parentId && byId.has(parentId) && !visited.has(parentId)) {
      visited.add(parentId)
      const parent = byId.get(parentId)
      names.unshift(parent.name)
      parentId = parent.parentId
    }
    return { value: category.id, label: names.join(' / ') }
  })
}

export function buildCategoryTree(state) {
  const nodes = new Map(
    state.categories.map((category) => [category.id, { ...category, children: [], recordCount: 0 }])
  )
  const roots = []
  for (const category of state.categories) {
    const node = nodes.get(category.id)
    const descendantIds = categoryIdsWithDescendants(state.categories, category.id)
    node.recordCount = state.records.filter((record) => descendantIds.has(record.categoryId)).length
    const parent = nodes.get(category.parentId)
    if (parent) {
      parent.children.push(node)
    } else {
      roots.push(node)
    }
  }
  return roots
}

export function queryRecords(state, query = {}) {
  const keyword = (query.keyword ?? '').trim().toLocaleLowerCase()
  const categoryIds = query.categoryId
    ? categoryIdsWithDescendants(state.categories, query.categoryId)
    : null
  const matches = state.records.filter((record) => {
    const matchesKeyword = `${record.name} ${record.code}`.toLocaleLowerCase().includes(keyword)
    const matchesCategory = !categoryIds || categoryIds.has(record.categoryId)
    const matchesStatus = !query.status || record.status === query.status
    return matchesKeyword && matchesCategory && matchesStatus
  })
  matches.sort(
    (left, right) =>
      right.updatedAt.localeCompare(left.updatedAt) || left.code.localeCompare(right.code)
  )
  const pageSize = Math.max(1, Math.floor(query.pageSize || 10))
  const pageCount = Math.max(1, Math.ceil(matches.length / pageSize))
  const page = Math.min(pageCount, Math.max(1, Math.floor(query.page || 1)))
  const start = (page - 1) * pageSize
  return { items: matches.slice(start, start + pageSize), total: matches.length, page, pageSize }
}

export function validateRecordInput(state, input, recordId = null) {
  requireValue(
    typeof input.name === 'string' && input.name.trim().length > 0,
    'examples.errors.nameRequired'
  )
  requireValue(input.name.trim().length <= 60, 'examples.errors.nameTooLong')
  requireValue(
    typeof input.code === 'string' && /^[A-Za-z0-9_-]{2,32}$/.test(input.code.trim()),
    'examples.errors.codeFormat'
  )
  const duplicateCode = state.records.some(
    (record) =>
      record.id !== recordId &&
      record.code.toLocaleLowerCase() === input.code.trim().toLocaleLowerCase()
  )
  requireValue(!duplicateCode, 'examples.errors.codeDuplicate')
  requireValue(
    state.categories.some((category) => category.id === input.categoryId),
    'examples.errors.categoryRequired'
  )
  requireValue(['enabled', 'disabled'].includes(input.status), 'examples.errors.statusRequired')
}

export function saveRecord(state, input, recordId = null, now = new Date()) {
  validateRecordInput(state, input, recordId)
  requireValue(
    !recordId || state.records.some((record) => record.id === recordId),
    'examples.errors.recordMissing'
  )
  const record = {
    id: recordId || crypto.randomUUID(),
    name: input.name.trim(),
    code: input.code.trim(),
    categoryId: input.categoryId,
    status: input.status,
    updatedAt: now.toISOString()
  }
  const records = recordId
    ? state.records.map((item) => (item.id === recordId ? record : item))
    : [record, ...state.records]
  return { ...state, records }
}

export function deleteRecord(state, recordId) {
  requireValue(
    state.records.some((record) => record.id === recordId),
    'examples.errors.recordMissing'
  )
  const records = state.records.filter((record) => record.id !== recordId)
  return { ...state, records }
}

export function saveCategory(state, input, categoryId = null) {
  const name = typeof input.name === 'string' ? input.name.trim() : ''
  const parentId = input.parentId || null
  requireValue(name.length > 0 && name.length <= 30, 'examples.errors.categoryName')
  requireValue(
    !parentId || state.categories.some((category) => category.id === parentId),
    'examples.errors.parentMissing'
  )
  const existing = state.categories.find((category) => category.id === categoryId)
  requireValue(!categoryId || existing, 'examples.errors.categoryMissing')
  requireValue(!existing || existing.parentId === parentId, 'examples.errors.categoryMove')
  const duplicateName = state.categories.some(
    (category) =>
      category.id !== categoryId &&
      category.parentId === parentId &&
      category.name.toLocaleLowerCase() === name.toLocaleLowerCase()
  )
  requireValue(!duplicateName, 'examples.errors.categoryDuplicate')
  const category = { id: categoryId || crypto.randomUUID(), name, parentId }
  const categories = categoryId
    ? state.categories.map((item) => (item.id === categoryId ? category : item))
    : [...state.categories, category]
  return { ...state, categories }
}

export function deleteCategory(state, categoryId) {
  requireValue(
    state.categories.some((category) => category.id === categoryId),
    'examples.errors.categoryMissing'
  )
  requireValue(
    !state.categories.some((category) => category.parentId === categoryId),
    'examples.errors.categoryHasChildren'
  )
  requireValue(
    !state.records.some((record) => record.categoryId === categoryId),
    'examples.errors.categoryHasRecords'
  )
  const categories = state.categories.filter((category) => category.id !== categoryId)
  return { ...state, categories }
}

function validateStoredState(state) {
  const invalidKey = 'examples.errors.storageInvalid'
  requireValue(
    state &&
      state.version === schemaVersion &&
      Array.isArray(state.categories) &&
      Array.isArray(state.records),
    invalidKey
  )
  const categoryIds = new Set()
  const siblingNames = new Set()
  for (const category of state.categories) {
    requireValue(
      category &&
        typeof category.id === 'string' &&
        category.id.length > 0 &&
        !categoryIds.has(category.id),
      invalidKey
    )
    requireValue(
      typeof category.name === 'string' &&
        category.name.trim().length > 0 &&
        category.name.length <= 30,
      invalidKey
    )
    requireValue(category.parentId === null || typeof category.parentId === 'string', invalidKey)
    const siblingName = JSON.stringify([
      category.parentId,
      category.name.trim().toLocaleLowerCase()
    ])
    requireValue(!siblingNames.has(siblingName), invalidKey)
    categoryIds.add(category.id)
    siblingNames.add(siblingName)
  }
  const byId = new Map(state.categories.map((category) => [category.id, category]))
  for (const category of state.categories) {
    requireValue(category.parentId === null || categoryIds.has(category.parentId), invalidKey)
    const visited = new Set([category.id])
    let parentId = category.parentId
    while (parentId) {
      requireValue(byId.has(parentId) && !visited.has(parentId), invalidKey)
      visited.add(parentId)
      parentId = byId.get(parentId).parentId
    }
  }
  const recordIds = new Set()
  const recordCodes = new Set()
  for (const record of state.records) {
    requireValue(
      record && typeof record.id === 'string' && record.id.length > 0 && !recordIds.has(record.id),
      invalidKey
    )
    requireValue(
      typeof record.name === 'string' && record.name.trim().length > 0 && record.name.length <= 60,
      invalidKey
    )
    requireValue(
      typeof record.code === 'string' && /^[A-Za-z0-9_-]{2,32}$/.test(record.code),
      invalidKey
    )
    requireValue(
      !recordCodes.has(record.code.toLocaleLowerCase()) && categoryIds.has(record.categoryId),
      invalidKey
    )
    requireValue(
      ['enabled', 'disabled'].includes(record.status) &&
        typeof record.updatedAt === 'string' &&
        Number.isFinite(Date.parse(record.updatedAt)),
      invalidKey
    )
    recordIds.add(record.id)
    recordCodes.add(record.code.toLocaleLowerCase())
  }
  return state
}

export function readState(storage, key) {
  const raw = storage.getItem(key)
  if (raw === null) {
    return createSeedState()
  }
  let parsed
  try {
    parsed = JSON.parse(raw)
  } catch {
    throw new ExampleDataError('examples.errors.storageInvalid')
  }
  return validateStoredState(parsed)
}

export function writeState(storage, key, state) {
  validateStoredState(state)
  const serialized = JSON.stringify(state)
  storage.setItem(key, serialized)
}
