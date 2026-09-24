import test from 'node:test'
import assert from 'node:assert/strict'
import { createDemoTransport, migrateMenuCatalog } from './model.mjs'
import { createSeedState } from './seeds.mjs'
import { createDefaultMenus, MENU_CATALOG_VERSION } from '../navigation/catalog.mjs'

const fixture = (initialState) => {
  const values = new Map()
  if (initialState) {
    values.set('demo-test', JSON.stringify(initialState))
  }
  const storage = {
    getItem: (key) => values.get(key) ?? null,
    setItem: (key, value) => values.set(key, value)
  }
  const open = () => createDemoTransport({ storage, storageKey: 'demo-test' })
  return { open, values, storage }
}

const legacyState = () => {
  const state = createSeedState()
  delete state.menuCatalogVersion
  state.menus = state.menus.filter((menu) => menu.id <= 4)
  state.roleMenus = { 1: [1, 2, 3, 4], 2: [1, 2, 4] }
  return state
}

test('旧版菜单补齐并持久化，保留用户编辑与原有权限', async () => {
  const state = legacyState()
  Object.assign(
    state.menus.find((menu) => menu.id === 2),
    {
      name: '已有成员',
      path: 'members',
      visible: false,
      sort: 42,
      icon: 'ep:key'
    }
  )
  const original = structuredClone(state.menus)
  const { open, values } = fixture(state)
  const menus = await open().request('GET', { url: '/system/menu/list' })
  assert.equal(menus.length, createDefaultMenus().length)
  assert.deepEqual(
    menus.filter((menu) => menu.id <= 4),
    original
  )
  const persisted = JSON.parse(values.get('demo-test'))
  assert.equal(persisted.menuCatalogVersion, MENU_CATALOG_VERSION)
  assert.deepEqual(persisted.roleMenus[2], [1, 2, 4])
  assert.deepEqual(new Set(persisted.roleMenus[1]), new Set(menus.map((menu) => menu.id)))
  assert.equal(migrateMenuCatalog(persisted), false)
  assert.deepEqual(await open().request('GET', { url: '/system/menu/list' }), menus)
})

test('迁移避让自定义 ID，新增子菜单关联实际目录 ID，后续新建继续使用空闲 ID', async () => {
  const state = legacyState()
  const custom = [
    { id: 5, name: '自定义页面', path: '/custom', parentId: 0, type: 2 },
    { id: 17, name: '自定义目录', path: '/custom-group', parentId: 0, type: 1 },
    { id: 1005, name: '已有条目', path: '/custom-second', parentId: 0, type: 2 }
  ]
  state.menus.push(...custom)
  const { open } = fixture(state)
  const menus = await open().request('GET', { url: '/system/menu/list' })
  assert.equal(menus.length, createDefaultMenus().length + custom.length)
  assert.equal(new Set(menus.map((menu) => menu.id)).size, menus.length)
  assert.deepEqual(
    menus.filter((menu) => custom.some((row) => row.id === menu.id)),
    custom
  )
  const examples = menus.find((menu) => menu.path === '/examples')
  assert.notEqual(examples.id, 17)
  assert.equal(menus.find((menu) => menu.path === 'crud').parentId, examples.id)
  const id = await open().request('POST', {
    url: '/system/menu/create',
    data: { name: '迁移后新建', path: '/new', parentId: 0, type: 2 }
  })
  assert.ok(id > Math.max(...menus.map((menu) => menu.id)))
})

test('已有默认菜单使用自定义 ID 时不重复添加，新增菜单复用其目录', () => {
  const state = legacyState()
  state.menus = state.menus.map((menu) => ({
    ...menu,
    id: menu.id + 80,
    parentId: menu.parentId ? menu.parentId + 80 : 0
  }))
  state.roleMenus[1] = [81, 82, 83, 84]
  migrateMenuCatalog(state)
  assert.equal(state.menus.length, createDefaultMenus().length)
  assert.equal(state.menus.find((menu) => menu.path === 'menu').parentId, 81)
  assert.equal(state.menus.find((menu) => menu.path === 'config').parentId, 84)
  assert.ok(!state.menus.some((menu) => menu.id <= 4))
})

test('迁移后主动删除的默认菜单刷新不会复活，迁移写入失败明确报错', async () => {
  const { open } = fixture(legacyState())
  const menus = await open().request('GET', { url: '/system/menu/list' })
  const home = menus.find((menu) => menu.path === '/home')
  await open().request('DELETE', { url: `/system/menu/delete?id=${home.id}` })
  const refreshed = await open().request('GET', { url: '/system/menu/list' })
  assert.ok(!refreshed.some((menu) => menu.path === '/home'))
  const failing = createDemoTransport({
    storage: {
      getItem: () => JSON.stringify(legacyState()),
      setItem: () => {
        throw new Error('QuotaExceeded')
      }
    },
    storageKey: 'legacy'
  })
  await assert.rejects(failing.request('GET', { url: '/system/menu/list' }), /QuotaExceeded/)
})

test('用户 CRUD 持久化，关系、筛选与分页可用，密码不落盘', async () => {
  const { open, values } = fixture()
  const demo = open()
  const id = await demo.request('POST', {
    url: '/system/user/create',
    data: {
      username: 'tester',
      nickname: '测试人员',
      deptId: 2,
      postIds: [2],
      password: 'not-a-real-secret'
    }
  })
  await demo.request('PUT', { url: '/system/user/update', data: { id, nickname: '更新人员' } })
  await demo.request('POST', {
    url: '/system/permission/assign-user-role',
    data: { userId: id, roleIds: [2] }
  })
  const fresh = open()
  const page = await fresh.request('GET', {
    url: '/system/user/page',
    params: { username: 'test', deptId: 1, pageSize: 1 }
  })
  assert.equal(page.total, 1)
  assert.equal(page.list[0].nickname, '更新人员')
  assert.equal(page.list[0].deptName, '运营组')
  assert.deepEqual(
    await fresh.request('GET', { url: `/system/permission/list-user-roles?userId=${id}` }),
    [2]
  )
  await fresh.request('PUT', {
    url: '/system/user/update-password',
    data: { id, password: 'demo-password' }
  })
  assert.ok((await open().request('GET', { url: `/system/user/get?id=${id}` })).passwordChangedAt)
  assert.ok(!values.get('demo-test').includes('demo-password'))
  assert.ok(!values.get('demo-test').includes('not-a-real-secret'))
  await fresh.request('DELETE', { url: '/system/user/delete-list', params: { ids: [id] } })
  await assert.rejects(open().request('GET', { url: `/system/user/get?id=${id}` }), /不存在/)
})

test('日期、状态、后续页筛选与 CSV 返回实际数据', async () => {
  const { open } = fixture()
  const demo = open()
  await demo.request('PUT', { url: '/system/user/update-status', data: { id: 2, status: 0 } })
  assert.equal(
    (await demo.request('GET', { url: '/system/user/page', params: { status: 0 } })).total,
    1
  )
  assert.equal(
    (await demo.request('GET', { url: '/system/user/page', params: { pageNo: 2, pageSize: 1 } }))
      .list[0].id,
    2
  )
  assert.equal(
    (
      await demo.request('GET', {
        url: '/system/user/page',
        params: { createTime: ['2027-01-01', '2027-12-31'] }
      })
    ).total,
    0
  )
  const blob = await demo.request('GET', {
    url: '/system/user/export-excel',
    params: { username: 'operator' }
  })
  assert.match(blob.type, /^text\/csv/)
  assert.match(await blob.text(), /operator/)
  assert.doesNotMatch(await blob.text(), /演示管理员/)
})

test('引用校验、重复键和树环路失败时不写入', async () => {
  const { open } = fixture()
  const demo = open()
  await assert.rejects(demo.request('DELETE', { url: '/system/dept/delete?id=2' }), /关联/)
  await assert.rejects(
    demo.request('PUT', { url: '/system/dept/update', data: { id: 1, parentId: 2 } }),
    /子节点/
  )
  await assert.rejects(
    demo.request('POST', {
      url: '/system/user/create',
      data: { username: 'admin', nickname: '重复' }
    }),
    /已存在/
  )
  assert.equal((await open().request('GET', { url: '/system/dept/get?id=1' })).parentId, 0)
})

test('菜单与数据范围分配持久化，错误日志可标记处理', async () => {
  const { open } = fixture()
  const demo = open()
  await demo.request('POST', {
    url: '/system/permission/assign-role-menu',
    data: { roleId: 2, menuIds: [1, 3] }
  })
  await demo.request('POST', {
    url: '/system/permission/assign-role-data-scope',
    data: { roleId: 2, dataScope: 2, dataScopeDeptIds: [1, 2] }
  })
  await demo.request('PUT', { url: '/infra/api-error-log/update-status?id=1&processStatus=1' })
  assert.deepEqual(
    await open().request('GET', { url: '/system/permission/list-role-menus?roleId=2' }),
    [1, 3]
  )
  assert.deepEqual(
    (await open().request('GET', { url: '/system/role/get?id=2' })).dataScopeDeptIds,
    [1, 2]
  )
  assert.equal(
    (
      await open().request('GET', {
        url: '/infra/api-error-log/page',
        params: { processStatus: 1 }
      })
    ).total,
    1
  )
})

test('本地文件上传刷新后可读、筛选、删除，超限被拒绝', async () => {
  const { open } = fixture()
  const demo = open()
  const form = new FormData()
  form.append('files', new Blob(['file contents'], { type: 'text/plain' }), 'notes.txt')
  form.append('relativePaths', 'folder/notes.txt')
  await demo.request('POST', { url: '/infra/file/upload-folder', data: form })
  const result = await open().request('GET', {
    url: '/infra/file/page',
    params: { path: 'folder/' }
  })
  assert.equal(result.total, 1)
  assert.equal(result.list[0].size, 13)
  const blob = await open().request('GET', { url: `/infra/file/download?id=${result.list[0].id}` })
  assert.equal(await blob.text(), 'file contents')
  await demo.request('DELETE', { url: `/infra/file/delete?id=${result.list[0].id}` })
  assert.equal(
    (await open().request('GET', { url: '/infra/file/page', params: { name: 'notes' } })).total,
    0
  )
  const oversized = new FormData()
  oversized.append('file', new Blob([new Uint8Array(2 * 1024 * 1024)]), 'large.bin')
  await assert.rejects(demo.request('POST', { url: '/infra/file/upload', data: oversized }), /2 MB/)
})

test('未知操作、导入与只读日志写入明确失败，存储失败不伪装成功', async () => {
  const { open } = fixture()
  for (const url of [
    '/system/user/import',
    '/system/user/get-import-template',
    '/unknown/endpoint',
    '/system/login-log/create'
  ]) {
    await assert.rejects(open().request('POST', { url, data: {} }), /不支持/)
  }
  const failing = createDemoTransport({
    storage: {
      getItem: () => null,
      setItem: () => {
        throw new Error('QuotaExceeded')
      }
    },
    storageKey: 'test'
  })
  await assert.rejects(
    failing.request('POST', { url: '/system/post/create', data: { name: '岗位', code: 'post' } }),
    /QuotaExceeded/
  )
})

test('文件配置主配置切换与测试真实写入本地文件，远程测试与凭据拒绝', async () => {
  const { open } = fixture()
  const demo = open()
  const id = await demo.request('POST', {
    url: '/infra/file-config/create',
    data: { name: '测试存储', storage: 1, config: {} }
  })
  await demo.request('PUT', { url: `/infra/file-config/update-master?id=${id}` })
  assert.equal((await open().request('GET', { url: '/infra/file-config/get?id=1' })).master, false)
  assert.equal(
    (await open().request('GET', { url: `/infra/file-config/get?id=${id}` })).master,
    true
  )
  const url = await demo.request('GET', { url: `/infra/file-config/test?id=${id}` })
  assert.match(url, /^blob:/)
  const files = await open().request('GET', {
    url: '/infra/file/page',
    params: { name: 'storage-test' }
  })
  assert.equal(files.total, 1)
  assert.equal(files.list[0].configId, id)
  await assert.rejects(
    demo.request('POST', {
      url: '/infra/file-config/create',
      data: { name: '存储', storage: 20, config: { accessSecret: 'do-not-save' } }
    }),
    /不保存存储凭据/
  )
  const remote = await demo.request('POST', {
    url: '/infra/file-config/create',
    data: { name: '外部存储', storage: 20, config: {} }
  })
  await assert.rejects(
    demo.request('GET', { url: `/infra/file-config/test?id=${remote}` }),
    /不连接远程/
  )
})

test('字典类型改名同步关联数据；创建状态默认启用', async () => {
  const { open } = fixture()
  const demo = open()
  const id = await demo.request('POST', {
    url: '/system/dict-type/create',
    data: { name: '颜色', type: 'demo_color' }
  })
  await demo.request('POST', {
    url: '/system/dict-data/create',
    data: { label: '蓝色', value: 'blue', dictType: 'demo_color' }
  })
  await demo.request('PUT', {
    url: '/system/dict-type/update',
    data: { id, type: 'demo_new_color' }
  })
  const rows = await open().request('GET', { url: '/system/dict-data/type?type=demo_new_color' })
  assert.equal(rows.length, 1)
  assert.equal(rows[0].status, 1)
  assert.equal(rows[0].label, '蓝色')
})
