import { createSeedState } from './seeds.mjs'
import { createDefaultMenus, MENU_CATALOG_VERSION } from '../navigation/catalog.mjs'

export const DEMO_NOTICE =
  '当前为浏览器本地演示：数据仅保存在当前浏览器，角色、权限和密码操作不代表服务端鉴权；请勿输入真实密码、密钥或敏感数据。'

const collections = {
  '/system/user': 'users',
  '/system/role': 'roles',
  '/system/menu': 'menus',
  '/system/dept': 'depts',
  '/system/post': 'posts',
  '/system/dict-type': 'dictTypes',
  '/system/dict-data': 'dictData',
  '/infra/config': 'configs',
  '/infra/file': 'files',
  '/infra/file-config': 'fileConfigs',
  '/system/login-log': 'loginLogs',
  '/system/operate-log': 'operateLogs',
  '/infra/api-access-log': 'apiAccessLogs',
  '/infra/api-error-log': 'apiErrorLogs'
}
const readOnly = new Set(['loginLogs', 'operateLogs', 'apiAccessLogs', 'apiErrorLogs'])
const copy = (value) => JSON.parse(JSON.stringify(value))
const now = () => new Date().toISOString()
const unsupported = () => {
  throw new Error('当前演示模式不支持此操作，请连接真实 API 后使用。')
}
const idsOf = (value) =>
  (Array.isArray(value) ? value : String(value ?? '').split(','))
    .filter((id) => String(id).length > 0)
    .map(Number)
const record = (rows, id) => {
  const row = rows.find((item) => item.id === Number(id))
  if (!row) {
    throw new Error('记录不存在或已被删除。')
  }
  return row
}

export function parseRequest(options) {
  const [path, query = ''] = options.url.split('?')
  const params = {}
  for (const [key, value] of new URLSearchParams(query)) {
    params[key] = key in params ? [].concat(params[key], value) : value
  }
  return { path, params: { ...params, ...options.params }, data: options.data ?? {} }
}

function filterRows(rows, params, state, collection) {
  return rows.filter((row) =>
    Object.entries(params).every(([key, value]) => {
      if (
        ['pageNo', 'pageSize', 'sortingFields'].includes(key) ||
        value === '' ||
        value == null ||
        (Array.isArray(value) && value.length === 0)
      ) {
        return true
      }
      if (Array.isArray(value) && /Time$/.test(key)) {
        const date = new Date(row[key]).getTime()
        return (
          Number.isFinite(date) &&
          (!value[0] || date >= new Date(value[0]).getTime()) &&
          (!value[1] || date <= new Date(value[1]).getTime())
        )
      }
      if (collection === 'users' && key === 'deptId') {
        const descendants = new Set([Number(value)])
        let previous = -1
        while (previous !== descendants.size) {
          previous = descendants.size
          state.depts
            .filter((dept) => descendants.has(dept.parentId))
            .forEach((dept) => descendants.add(dept.id))
        }
        return descendants.has(Number(row.deptId))
      }
      if (key === 'duration') {
        return Number(row.duration) >= Number(value)
      }
      if (key === 'dictType') {
        return row.dictType === value
      }
      if (typeof row[key] === 'number' || typeof row[key] === 'boolean') {
        return String(row[key]) === String(value)
      }
      return String(row[key] ?? '')
        .toLowerCase()
        .includes(String(value).toLowerCase())
    })
  )
}

function csvBlob(rows) {
  const columns = [...new Set(rows.flatMap((row) => Object.keys(row)))].filter(
    (key) => !['password', 'url'].includes(key)
  )
  const cell = (value) => {
    let text =
      typeof value === 'object' && value !== null ? JSON.stringify(value) : String(value ?? '')
    if (/^[=+@\-\t\r]/.test(text)) {
      text = `'${text}`
    }
    return `"${text.replaceAll('"', '""')}"`
  }
  const content = [columns, ...rows.map((row) => columns.map((key) => row[key]))]
    .map((row) => row.map(cell).join(','))
    .join('\r\n')
  return new Blob(['\uFEFF', content], { type: 'text/csv;charset=utf-8' })
}

function validateRow(state, collection, row) {
  const fields = {
    users: ['username', 'nickname'],
    roles: ['name', 'code'],
    menus: ['name'],
    depts: ['name'],
    posts: ['name', 'code'],
    dictTypes: ['name', 'type'],
    dictData: ['label', 'value', 'dictType'],
    configs: ['name', 'key', 'value'],
    fileConfigs: ['name']
  }
  for (const key of fields[collection] ?? []) {
    if (row[key] == null || String(row[key]).trim() === '') {
      throw new Error(`字段 ${key} 不能为空。`)
    }
  }
  const unique = {
    users: 'username',
    roles: 'code',
    posts: 'code',
    dictTypes: 'type',
    configs: 'key'
  }[collection]
  if (
    unique &&
    state[collection].some((item) => item.id !== row.id && item[unique] === row[unique])
  ) {
    throw new Error(`字段 ${unique} 已存在。`)
  }
  if (collection === 'users') {
    if (row.deptId) {
      record(state.depts, row.deptId)
    }
    idsOf(row.postIds).forEach((id) => record(state.posts, id))
  }
  if (collection === 'dictData' && !state.dictTypes.some((item) => item.type === row.dictType)) {
    throw new Error('字典类型不存在。')
  }
  if (['menus', 'depts'].includes(collection) && Number(row.parentId)) {
    let ancestor = record(state[collection], row.parentId)
    const visited = new Set([row.id])
    while (ancestor) {
      if (visited.has(ancestor.id)) {
        throw new Error('不能将自身或子节点设为上级。')
      }
      visited.add(ancestor.id)
      ancestor = state[collection].find((item) => item.id === Number(ancestor.parentId))
    }
  }
  if (collection === 'fileConfigs') {
    const config = row.config ?? {}
    if (['password', 'accessKey', 'accessSecret'].some((key) => config[key])) {
      throw new Error('演示模式不保存存储凭据，请连接真实 API 后配置。')
    }
  }
}

function validateDelete(state, collection, ids) {
  const references = {
    depts:
      state.depts.some((row) => ids.includes(row.parentId) && !ids.includes(row.id)) ||
      state.users.some((row) => ids.includes(row.deptId)),
    menus: state.menus.some((row) => ids.includes(row.parentId) && !ids.includes(row.id)),
    posts: state.users.some((row) => idsOf(row.postIds).some((id) => ids.includes(id))),
    dictTypes: state.dictData.some((row) =>
      state.dictTypes.some((type) => ids.includes(type.id) && type.type === row.dictType)
    ),
    fileConfigs:
      state.files.some((row) => ids.includes(row.configId)) ||
      state.fileConfigs.some((row) => ids.includes(row.id) && row.master)
  }
  if (references[collection]) {
    throw new Error('记录仍有下级或关联数据，请先解除关联。')
  }
}

async function fileDataUrl(file) {
  const bytes = new Uint8Array(await file.arrayBuffer())
  let binary = ''
  for (const byte of bytes) {
    binary += String.fromCharCode(byte)
  }
  return `data:${file.type || 'application/octet-stream'};base64,${btoa(binary)}`
}

function blobFromUrl(url) {
  const [header, body] = url.split(',')
  const content = header.includes(';base64')
    ? Uint8Array.from(atob(body), (character) => character.charCodeAt(0))
    : decodeURIComponent(body)
  return new Blob([content], { type: header.slice(5).split(';')[0] || 'application/octet-stream' })
}

export function migrateMenuCatalog(state) {
  if (Number(state.menuCatalogVersion ?? 0) >= MENU_CATALOG_VERSION) {
    return false
  }
  const defaults = createDefaultMenus()
  const defaultPaths = new Map()
  const actualIds = new Map()
  const matched = new Set()
  const usedIds = new Set(state.menus.map((row) => Number(row.id)))
  let nextId = Math.max(1000, Number(state.nextId) || 0, ...usedIds) + 1
  const added = []
  for (const menu of defaults) {
    const parentPath = defaultPaths.get(menu.parentId) ?? ''
    const path = menu.path.startsWith('/') ? menu.path : `${parentPath}/${menu.path}`
    defaultPaths.set(menu.id, path)
    const parentId = actualIds.get(menu.parentId) ?? 0
    const available = state.menus.filter((row) => !matched.has(row.id))
    const existing =
      available.find(
        (row) =>
          Number(row.type) === menu.type &&
          (row.path === path ||
            (row.path === menu.path && Number(row.parentId) === parentId) ||
            (menu.component && row.component === menu.component))
      ) ?? available.find((row) => menu.id <= 4 && row.id === menu.id)
    if (existing) {
      actualIds.set(menu.id, existing.id)
      matched.add(existing.id)
      continue
    }
    const id = usedIds.has(menu.id) ? nextId++ : menu.id
    const addedMenu = { ...menu, id, parentId }
    state.menus.push(addedMenu)
    usedIds.add(id)
    matched.add(id)
    actualIds.set(menu.id, id)
    added.push(id)
  }
  const admin =
    state.roles.find((role) => role.code === 'admin') ?? state.roles.find((role) => role.id === 1)
  if (admin) {
    state.roleMenus ??= {}
    state.roleMenus[admin.id] = [...new Set([...(state.roleMenus[admin.id] ?? []), ...added])]
  }
  state.nextId = Math.max(Number(state.nextId) || 0, nextId)
  state.menuCatalogVersion = MENU_CATALOG_VERSION
  return true
}

export function createDemoTransport({ storage, storageKey }) {
  const fileUrls = new Map()
  let queue = Promise.resolve()
  const read = () => {
    const saved = storage.getItem(storageKey)
    if (!saved) {
      return createSeedState()
    }
    const state = JSON.parse(saved)
    if (
      state.version !== 1 ||
      !Object.values(collections).every((key) => Array.isArray(state[key]))
    ) {
      throw new Error('本地演示数据格式不兼容，请清除本应用的演示存储后重试。')
    }
    if (migrateMenuCatalog(state)) {
      storage.setItem(storageKey, JSON.stringify(state))
    }
    return state
  }
  const present = (rows, collection, state) =>
    rows.map((row) => {
      const result = copy(row)
      if (collection === 'users') {
        result.deptName = state.depts.find((dept) => dept.id === row.deptId)?.name ?? ''
      }
      if (collection === 'files' && row.url.startsWith('data:')) {
        if (!fileUrls.has(row.url)) {
          fileUrls.set(row.url, URL.createObjectURL(blobFromUrl(row.url)))
        }
        result.url = fileUrls.get(row.url)
      }
      return result
    })
  const execute = async (method, options) => {
    const { path, params, data } = parseRequest(options)
    const state = read()
    const save = (value) => {
      storage.setItem(storageKey, JSON.stringify(state))
      return value
    }
    const permission = '/system/permission/'
    if (path.startsWith(permission)) {
      const action = path.slice(permission.length)
      if (method === 'GET' && action === 'list-user-roles') {
        record(state.users, params.userId)
        return copy(state.userRoles[params.userId] ?? [])
      }
      if (method === 'GET' && action === 'list-role-menus') {
        record(state.roles, params.roleId)
        return copy(state.roleMenus[params.roleId] ?? [])
      }
      if (method !== 'POST') {
        return unsupported()
      }
      if (action === 'assign-user-role') {
        record(state.users, data.userId)
        const ids = idsOf(data.roleIds)
        ids.forEach((id) => record(state.roles, id))
        state.userRoles[data.userId] = ids
        return save(true)
      }
      const role = record(state.roles, data.roleId)
      if (action === 'assign-role-menu') {
        const ids = idsOf(data.menuIds)
        ids.forEach((id) => record(state.menus, id))
        state.roleMenus[role.id] = ids
        return save(true)
      }
      if (action === 'assign-role-data-scope') {
        if (![1, 2, 3, 4, 5].includes(Number(data.dataScope))) {
          throw new Error('数据范围无效。')
        }
        const ids = idsOf(data.dataScopeDeptIds)
        ids.forEach((id) => record(state.depts, id))
        Object.assign(role, { dataScope: Number(data.dataScope), dataScopeDeptIds: ids })
        return save(true)
      }
      return unsupported()
    }
    const slash = path.lastIndexOf('/')
    const base = path.slice(0, slash)
    const action = path.slice(slash + 1)
    const collection = collections[base]
    if (!collection || ['import', 'get-import-template', 'presigned-url'].includes(action)) {
      return unsupported()
    }
    const rows = state[collection]
    if (
      collection === 'files' &&
      method === 'POST' &&
      ['upload', 'upload-folder'].includes(action)
    ) {
      if (!(data instanceof FormData)) {
        throw new Error('请选择需要上传的文件。')
      }
      const files = [...data.getAll('files'), ...data.getAll('file')].filter(
        (file) => file instanceof Blob
      )
      if (!files.length) {
        throw new Error('请选择需要上传的文件。')
      }
      const size =
        files.reduce((sum, file) => sum + file.size, 0) +
        rows.reduce((sum, file) => sum + file.size, 0)
      if (size > 2 * 1024 * 1024) {
        throw new Error('演示文件总量限制为 2 MB，请删除部分文件后重试。')
      }
      const config = state.fileConfigs.find((item) => item.master)
      if (!config) {
        throw new Error('请先设置一个主文件配置。')
      }
      if (![1, 10].includes(config.storage)) {
        throw new Error('演示上传只支持浏览器本地存储，请选择数据库或本地存储配置。')
      }
      const paths = data.getAll('relativePaths')
      const added = []
      for (const [index, file] of files.entries()) {
        const id = Math.max(1000, state.nextId++)
        state.nextId = id + 1
        const name = file.name || `file-${id}`
        const pathValue = String(paths[index] || name).replaceAll('\\', '/')
        if (pathValue.startsWith('/') || pathValue.split('/').includes('..')) {
          throw new Error('文件路径无效。')
        }
        const item = {
          id,
          configId: config.id,
          name,
          path: data.get('keepOriginalName') === 'false' ? `${id}-${name}` : pathValue,
          url: await fileDataUrl(file),
          size: file.size,
          type: file.type || 'application/octet-stream',
          createTime: now()
        }
        rows.unshift(item)
        added.push(item)
      }
      save(true)
      const urls = present(added, collection, state).map((file) => file.url)
      return action === 'upload' ? urls[0] : urls
    }
    if (method === 'GET') {
      if (action === 'get') {
        return present([record(rows, params.id)], collection, state)[0]
      }
      if (action === 'type' && collection === 'dictData') {
        return copy(rows.filter((row) => row.dictType === params.type))
      }
      if (action === 'get-value-by-key' && collection === 'configs') {
        const config = rows.find((row) => row.key === params.key)
        if (!config) {
          throw new Error('参数键名不存在。')
        }
        return config.value
      }
      if (action === 'download' && collection === 'files') {
        return blobFromUrl(record(rows, params.id).url)
      }
      if (action === 'test' && collection === 'fileConfigs') {
        const config = record(rows, params.id)
        if (![1, 10].includes(config.storage)) {
          throw new Error('演示模式只支持浏览器本地存储测试，不连接远程文件服务。')
        }
        const blob = new Blob(['浏览器本地演示存储测试成功；未连接任何远程存储。'], {
          type: 'text/plain;charset=utf-8'
        })
        const id = Math.max(1000, state.nextId)
        state.nextId = id + 1
        const file = {
          id,
          configId: config.id,
          name: 'storage-test.txt',
          path: `demo/storage-test-${id}.txt`,
          url: await fileDataUrl(blob),
          size: blob.size,
          type: blob.type,
          createTime: now()
        }
        state.files.unshift(file)
        save(true)
        return present([record(read().files, id)], 'files', state)[0].url
      }
      const filtered = filterRows(rows, params, state, collection)
      if (action === 'export-excel') {
        return csvBlob(filtered)
      }
      if (action === 'simple-list') {
        return present(
          rows.filter((row) => row.status == null || row.status === 1),
          collection,
          state
        )
      }
      if (action === 'list') {
        return present(filtered, collection, state)
      }
      if (action === 'page') {
        const pageNo = Math.max(1, Math.floor(Number(params.pageNo) || 1))
        const pageSize = Math.max(1, Math.min(1000, Math.floor(Number(params.pageSize) || 10)))
        return {
          list: present(
            filtered.slice((pageNo - 1) * pageSize, pageNo * pageSize),
            collection,
            state
          ),
          total: filtered.length
        }
      }
      return unsupported()
    }
    if (method === 'PUT' && action === 'update-status' && collection === 'apiErrorLogs') {
      const status = Number(params.processStatus)
      if (![0, 1, 2].includes(status)) {
        throw new Error('处理状态无效。')
      }
      Object.assign(record(rows, params.id), {
        processStatus: status,
        processTime: now(),
        processUserId: 1
      })
      return save(true)
    }
    if (readOnly.has(collection)) {
      return unsupported()
    }
    if (method === 'PUT' && action === 'update-password' && collection === 'users') {
      if (typeof data.password !== 'string' || data.password.length < 6) {
        throw new Error('演示密码至少需要 6 个字符。')
      }
      record(rows, data.id).passwordChangedAt = now()
      return save(true)
    }
    if (method === 'PUT' && action === 'update-status' && ['users', 'roles'].includes(collection)) {
      if (![0, 1].includes(Number(data.status))) {
        throw new Error('状态无效。')
      }
      record(rows, data.id).status = Number(data.status)
      return save(true)
    }
    if (method === 'PUT' && action === 'update-master' && collection === 'fileConfigs') {
      const target = record(rows, params.id)
      rows.forEach((row) => {
        row.master = row.id === target.id
      })
      return save(true)
    }
    if (method === 'POST' && action === 'create' && collection !== 'files') {
      const id = Math.max(1000, state.nextId)
      state.nextId = id + 1
      const row = { status: 1, sort: 1, type: 2, remark: '', ...copy(data), id, createTime: now() }
      delete row.password
      if (collection === 'roles') {
        Object.assign(row, { dataScope: 5, dataScopeDeptIds: [] })
      }
      if (collection === 'fileConfigs') {
        row.master = false
      }
      validateRow(state, collection, row)
      rows.unshift(row)
      return save(id)
    }
    if (method === 'PUT' && action === 'update' && collection !== 'files') {
      const row = record(rows, data.id)
      const updated = { ...row, ...copy(data), id: row.id, createTime: row.createTime }
      delete updated.password
      validateRow(state, collection, updated)
      if (collection === 'dictTypes' && row.type !== updated.type) {
        state.dictData
          .filter((item) => item.dictType === row.type)
          .forEach((item) => {
            item.dictType = updated.type
          })
      }
      Object.assign(row, updated)
      return save(true)
    }
    if (method === 'DELETE' && ['delete', 'delete-list'].includes(action)) {
      const ids = action === 'delete' ? [Number(params.id)] : idsOf(params.ids)
      if (!ids.length) {
        throw new Error('请选择需要删除的记录。')
      }
      ids.forEach((id) => record(rows, id))
      validateDelete(state, collection, ids)
      state[collection] = rows.filter((row) => !ids.includes(row.id))
      if (collection === 'users') {
        ids.forEach((id) => {
          delete state.userRoles[id]
        })
      }
      if (collection === 'roles') {
        ids.forEach((id) => {
          delete state.roleMenus[id]
        })
        Object.keys(state.userRoles).forEach((id) => {
          state.userRoles[id] = state.userRoles[id].filter((roleId) => !ids.includes(roleId))
        })
      }
      if (collection === 'menus') {
        Object.keys(state.roleMenus).forEach((id) => {
          state.roleMenus[id] = state.roleMenus[id].filter((menuId) => !ids.includes(menuId))
        })
      }
      save(true)
      if (collection === 'files') {
        rows
          .filter((row) => ids.includes(row.id))
          .forEach((row) => {
            const url = fileUrls.get(row.url)
            if (url) {
              URL.revokeObjectURL(url)
              fileUrls.delete(row.url)
            }
          })
      }
      return true
    }
    return unsupported()
  }
  return {
    request(method, options) {
      const result = queue.then(() => execute(method, options))
      queue = result.then(
        () => undefined,
        () => undefined
      )
      return result
    }
  }
}
