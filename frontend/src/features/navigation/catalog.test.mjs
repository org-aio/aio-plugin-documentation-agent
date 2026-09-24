import test from 'node:test'
import assert from 'node:assert/strict'
import {
  buildNavigation,
  collectAccessiblePaths,
  collectNoCachePaths,
  createDefaultMenus,
  pageMetadata
} from './catalog.mjs'

const availablePaths = Object.keys(pageMetadata)
const leaves = (items) => items.flatMap((item) => (item.children ? leaves(item.children) : [item]))

test('首页、系统、基础设施、示例与业务菜单完整，隐藏详情不进入导航', () => {
  const menus = createDefaultMenus()
  const navigation = buildNavigation(menus, availablePaths)
  assert.equal(menus.length, 40)
  // 根级可见目录：首页、系统、基础设施、示例，以及 boxun 的原始数据/生成数据。
  assert.deepEqual(
    navigation.map((item) => item.path),
    [
      '/home',
      '/system',
      '/infra',
      '/examples',
      '/boxun/historical-weather',
      '/boxun/delegation-order-context',
      '/boxun/project-info',
      '/boxun/raw-data',
      '/boxun/generated-data',
      '/boxun/commission-order',
      '/boxun/template-management',
      '/boxun/payment-settings'
    ]
  )
  assert.deepEqual(
    navigation.slice(1).map((item) => (item.children ?? []).length),
    [8, 6, 2, 0, 0, 0, 2, 5, 0, 0, 0]
  )
  assert.deepEqual(
    leaves(navigation)
      .map((item) => item.path)
      .sort(),
    Object.entries(pageMetadata)
      .filter(([, value]) => !value.hidden)
      .map(([path]) => path)
      .sort()
  )
  assert.equal(menus.find((menu) => menu.id === 1).path, '/system')
  assert.equal(menus.find((menu) => menu.id === 2).path, 'user')
  assert.equal(menus.find((menu) => menu.id === 3).path, 'role')
  assert.equal(menus.find((menu) => menu.id === 4).path, '/infra')
  menus[0].name = '新的名称'
  assert.equal(createDefaultMenus()[0].name, '首页')
})

test('导航使用编辑后的名称、图标、排序和路径，过滤隐藏、停用、按钮与未注册页面', () => {
  const menus = createDefaultMenus()
  Object.assign(
    menus.find((menu) => menu.id === 2),
    { name: '成员', icon: 'ep:key', sort: 9 }
  )
  menus.find((menu) => menu.id === 3).sort = 0
  menus.find((menu) => menu.id === 5).visible = false
  menus.find((menu) => menu.id === 4).status = 0
  menus.find((menu) => menu.id === 6).type = 3
  menus.find((menu) => menu.id === 7).path = 'unregistered'
  menus.find((menu) => menu.id === 8).path = '/custom-posts'
  const navigation = buildNavigation(menus, [...availablePaths, '/custom-posts'])
  // 系统与示例目录仍按编辑后的顺序出现；boxun 业务菜单未受编辑影响，保持可见。
  assert.deepEqual(
    navigation
      .filter((item) => ['/system', '/examples'].includes(item.path))
      .map((item) => item.path),
    ['/system', '/examples']
  )
  const system = navigation[0].children
  assert.equal(system[0].path, '/system/role')
  assert.deepEqual(system.at(-1), { path: '/system/user', title: '成员', icon: 'ep:key' })
  assert.ok(system.some((item) => item.path === '/custom-posts'))
  assert.ok(!system.some((item) => ['/system/menu', '/system/unregistered'].includes(item.path)))
})

test('权限接口嵌套菜单可导航，孤立记录、环路和空目录不会出现', () => {
  const menus = [
    {
      id: 1,
      name: '系统',
      path: '/system',
      children: [
        { id: 2, name: '用户', path: 'user', sort: 2 },
        { id: 3, name: '角色', path: 'role', sort: 1 }
      ]
    },
    { id: 4, name: '空目录', path: '/empty', type: 1 },
    { id: 5, name: '孤立', path: '/home', parentId: 99 },
    { id: 6, name: '环路一', path: '/loop', parentId: 7 },
    { id: 7, name: '环路二', path: '/loop-again', parentId: 6 }
  ]
  const navigation = buildNavigation(menus, availablePaths)
  assert.deepEqual(
    navigation.map((item) => item.path),
    ['/system']
  )
  assert.deepEqual(
    navigation[0].children.map((item) => item.path),
    ['/system/role', '/system/user']
  )
})

test('真实后端日志与文件目录保留层级，六个叶子按注册组件映射到公共页面', () => {
  const menus = [
    {
      id: 1,
      name: '系统管理',
      path: '/system',
      type: 1,
      children: [
        {
          id: 108,
          name: '审计日志',
          path: 'log',
          type: 1,
          children: [
            {
              id: 500,
              name: '操作日志',
              path: 'operate-log',
              component: 'system/operatelog/index'
            },
            { id: 501, name: '登录日志', path: 'login-log', component: '/system/loginlog/index' }
          ]
        }
      ]
    },
    {
      id: 2,
      name: '基础设施',
      path: '/infra',
      type: 1,
      children: [
        {
          id: 1083,
          name: 'API 日志',
          path: 'log',
          type: 2,
          children: [
            {
              id: 1078,
              name: '访问日志',
              path: 'api-access-log',
              component: 'infra/apiAccessLog/index'
            },
            {
              id: 1084,
              name: '错误日志',
              path: 'api-error-log',
              component: 'infra/apiErrorLog/index'
            }
          ]
        },
        {
          id: 1243,
          name: '文件管理',
          path: 'file',
          type: 2,
          children: [
            {
              id: 1237,
              name: '文件配置',
              path: 'file-config',
              component: 'infra/fileConfig/index'
            },
            { id: 1090, name: '文件列表', path: 'file', component: 'infra/file/index' }
          ]
        }
      ]
    }
  ]
  const navigation = buildNavigation(menus, availablePaths)
  assert.equal(navigation[0].children[0].path, '/system/log')
  assert.deepEqual(
    navigation[1].children.map((item) => item.path),
    ['/infra/log', '/infra/file']
  )
  assert.deepEqual(
    leaves(navigation).map((item) => item.path),
    [
      '/system/operatelog',
      '/system/loginlog',
      '/infra/apiAccessLog',
      '/infra/apiErrorLog',
      '/infra/fileConfig',
      '/infra/file'
    ]
  )
})

test('已注册后端路径优先于组件，组件映射仍受显隐、状态和父节点过滤', () => {
  const menus = [
    { id: 1, name: '自定义入口', path: '/custom-users', component: 'system/user/index' },
    { id: 2, name: '隐藏', path: '/hidden-users', component: 'system/user/index', visible: false },
    { id: 3, name: '禁用', path: '/disabled-users', component: 'system/user/index', status: 0 },
    { id: 4, name: '按钮', path: '/button-users', component: 'system/user/index', type: 3 },
    { id: 5, name: '孤立', path: '/orphan-users', component: 'system/user/index', parentId: 99 },
    {
      id: 6,
      name: '禁用目录',
      path: '/disabled-group',
      status: 0,
      children: [{ id: 7, name: '子项', path: 'users', component: 'system/user/index' }]
    },
    { id: 8, name: '空目录', path: '/empty-group', component: 'system/user/index', type: 1 }
  ]
  const navigation = buildNavigation(menus, [...availablePaths, '/custom-users'])
  assert.deepEqual(navigation, [
    { path: '/custom-users', title: '自定义入口', icon: 'ep:document' }
  ])
})

test('未注册或包含外部地址、路径穿越与编码的组件不会成为导航入口', () => {
  const components = [
    'unsupported/page/index',
    'https://outside.invalid/system/user/index',
    '//outside.invalid/system/user/index',
    'javascript:system/user/index',
    'system/../user/index',
    'system/./user/index',
    'system\\user\\index',
    'system/%2e%2e/user/index',
    'system/user/index?redirect=outside',
    'system/user/index#outside',
    'system/user\n/index',
    null
  ]
  const menus = components.map((component, id) => ({
    id: id + 1,
    name: '不支持的入口',
    path: `/unregistered-${id}`,
    component
  }))
  const untrustedPaths = ['/system/../user', '/system/./user', '/system/%2e%2e/user']
  assert.deepEqual(buildNavigation(menus, [...availablePaths, ...untrustedPaths]), [])
})

test('可访问路径来自启用菜单，目录只贡献子级，组件路径可映射', () => {
  const menus = createDefaultMenus()
  const paths = collectAccessiblePaths(menus, availablePaths)
  assert.ok(paths.has('/home'))
  assert.ok(paths.has('/system/user'))
  assert.ok(!paths.has('/system'))
  assert.ok(!paths.has('/system/dict/data'))
  menus.find((menu) => menu.id === 3).status = 0
  assert.ok(!collectAccessiblePaths(menus, availablePaths).has('/system/role'))
  assert.equal(collectAccessiblePaths(null, availablePaths), null)
  assert.equal(collectAccessiblePaths([], availablePaths), null)
})

test('隐藏子页跟随组件路径授权，未注册路径不进入集合', () => {
  const menus = [
    { id: 1, name: '字典', path: 'dict', component: 'system/dict/index', parentId: 0, type: 2 }
  ]
  const paths = collectAccessiblePaths(menus, ['/system/dict', '/system/dict/data'])
  assert.ok(paths.has('/system/dict'))
  assert.ok(!paths.has('/system/dict/data'))
})

test('keepAlive:false 的叶子进入禁用缓存集合，目录不受影响', () => {
  const menus = createDefaultMenus()
  const none = collectNoCachePaths(menus, availablePaths)
  assert.equal(none.size, 0)
  menus.find((menu) => menu.id === 2).keepAlive = false
  const noCache = collectNoCachePaths(menus, availablePaths)
  assert.ok(noCache.has('/system/user'))
  assert.ok(!noCache.has('/system'))
})

test('分组页面同时提供路径分组与显示翻译键，避免面包屑显示原始路径', () => {
  const grouped = Object.entries(pageMetadata).filter(([, value]) => value.group)
  assert.ok(grouped.length > 0)
  for (const [path, value] of grouped) {
    assert.ok(value.groupTitleKey, `${path} 缺少 groupTitleKey，面包屑会把 group 路径当作翻译键`)
  }
  const block = pageMetadata['/boxun/block-retention-ledger']
  assert.equal(block.group, 'boxun/generated-data')
  assert.equal(block.groupTitleKey, 'app.generated-data')
})
