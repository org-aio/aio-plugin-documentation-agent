const created = '2026-01-15 09:30:00'

export const MENU_CATALOG_VERSION = 4

// 页面文件仍是路由来源；这里统一维护默认菜单与路由展示信息。
const catalog = [
  { id: 5, path: '/home', name: '首页', titleKey: 'app.home', icon: 'ep:home-filled', sort: 0 },
  {
    id: 1,
    path: '/system',
    name: '系统管理',
    titleKey: 'app.system',
    icon: 'ep:setting',
    sort: 1,
    type: 1
  },
  {
    id: 2,
    path: '/system/user',
    name: '用户管理',
    titleKey: 'system.user',
    icon: 'ep:user',
    sort: 1,
    parentId: 1,
    permission: 'system:user:query'
  },
  {
    id: 3,
    path: '/system/role',
    name: '角色管理',
    titleKey: 'system.role',
    icon: 'ep:user-filled',
    sort: 2,
    parentId: 1,
    permission: 'system:role:query'
  },
  {
    id: 6,
    path: '/system/menu',
    name: '菜单管理',
    titleKey: 'system.menu',
    icon: 'ep:menu',
    sort: 3,
    parentId: 1,
    permission: 'system:menu:query'
  },
  {
    id: 7,
    path: '/system/dept',
    name: '部门管理',
    titleKey: 'system.dept',
    icon: 'ep:tickets',
    sort: 4,
    parentId: 1,
    permission: 'system:dept:query'
  },
  {
    id: 8,
    path: '/system/post',
    name: '岗位管理',
    titleKey: 'system.post',
    icon: 'ep:document',
    sort: 5,
    parentId: 1,
    permission: 'system:post:query'
  },
  {
    id: 9,
    path: '/system/dict',
    name: '字典管理',
    titleKey: 'system.dict',
    icon: 'ep:grid',
    sort: 6,
    parentId: 1,
    permission: 'system:dict:query'
  },
  {
    id: 10,
    path: '/system/loginlog',
    name: '登录日志',
    titleKey: 'system.loginlog',
    icon: 'ep:document',
    sort: 7,
    parentId: 1,
    permission: 'system:login-log:query'
  },
  {
    id: 11,
    path: '/system/operatelog',
    name: '操作日志',
    titleKey: 'system.operatelog',
    icon: 'ep:document',
    sort: 8,
    parentId: 1,
    permission: 'system:operate-log:query'
  },
  {
    id: 4,
    path: '/infra',
    name: '基础设施',
    titleKey: 'app.infra',
    icon: 'ep:tools',
    sort: 2,
    type: 1
  },
  {
    id: 12,
    path: '/infra/config',
    name: '参数配置',
    titleKey: 'infra.config',
    icon: 'ep:setting',
    sort: 1,
    parentId: 4,
    permission: 'infra:config:query'
  },
  {
    id: 39,
    path: '/boxun/mail-account',
    name: '邮件配置',
    titleKey: 'infra.mailAccount',
    icon: 'ep:message',
    sort: 2,
    parentId: 4,
    permission: 'system:mail-account:query'
  },
  {
    id: 13,
    path: '/infra/file',
    name: '文件管理',
    titleKey: 'infra.file',
    icon: 'ep:folder',
    sort: 3,
    parentId: 4,
    permission: 'infra:file:query'
  },
  {
    id: 14,
    path: '/infra/fileConfig',
    name: '文件配置',
    titleKey: 'infra.fileConfig',
    icon: 'ep:setting',
    sort: 4,
    parentId: 4,
    permission: 'infra:file-config:query'
  },
  {
    id: 15,
    path: '/infra/apiAccessLog',
    name: 'API 访问日志',
    titleKey: 'infra.apiAccessLog',
    icon: 'ep:document',
    sort: 5,
    parentId: 4,
    permission: 'infra:api-access-log:query'
  },
  {
    id: 16,
    path: '/infra/apiErrorLog',
    name: 'API 错误日志',
    titleKey: 'infra.apiErrorLog',
    icon: 'ep:document',
    sort: 6,
    parentId: 4,
    permission: 'infra:api-error-log:query'
  },
  {
    id: 17,
    path: '/examples',
    name: '功能示例',
    titleKey: 'app.examples',
    icon: 'ep:grid',
    sort: 3,
    type: 1
  },
  {
    id: 18,
    path: '/examples/crud',
    name: '普通 CRUD',
    titleKey: 'examples.crud.title',
    icon: 'ep:grid',
    sort: 1,
    parentId: 17
  },
  {
    id: 19,
    path: '/examples/tree-table',
    name: '左树右表',
    titleKey: 'examples.treeTable.title',
    icon: 'ep:tickets',
    sort: 2,
    parentId: 17
  },
  // 与旧版 boxun-vue 的 sys_permission 可见菜单保持一致。
  {
    id: 20,
    path: '/boxun/historical-weather',
    name: '历史天气',
    titleKey: 'boxun.historicalWeather',
    icon: 'ep:cloudy',
    sort: 4
  },
  {
    id: 21,
    path: '/boxun/delegation-order-context',
    name: '样品配置',
    titleKey: 'boxun.delegationOrderContext',
    icon: 'ep:tickets',
    sort: 5
  },
  {
    id: 22,
    path: '/boxun/project-info',
    name: '我的项目',
    titleKey: 'boxun.projectInfo',
    icon: 'ep:folder',
    sort: 6
  },
  {
    id: 23,
    path: '/boxun/raw-data',
    name: '原始数据',
    titleKey: 'app.raw-data',
    icon: 'ep:grid',
    sort: 7,
    type: 1
  },
  {
    id: 24,
    path: '/boxun/commercial-concrete-ledger',
    name: '商混台账',
    titleKey: 'boxun.commercialConcreteLedger',
    icon: 'ep:grid',
    sort: 1,
    parentId: 23
  },
  {
    id: 25,
    path: '/boxun/raw-material-ledger',
    name: '原材料进场台账',
    titleKey: 'boxun.rawMaterialLedger',
    icon: 'ep:grid',
    sort: 2,
    parentId: 23
  },
  {
    id: 26,
    path: '/boxun/generated-data',
    name: '生成数据',
    titleKey: 'app.generated-data',
    icon: 'ep:tickets',
    sort: 8,
    type: 1
  },
  {
    id: 27,
    path: '/boxun/witness-record',
    name: '见证记录',
    titleKey: 'boxun.witnessRecord',
    icon: 'ep:document',
    sort: 1,
    parentId: 26
  },
  {
    id: 28,
    path: '/boxun/bystander-record',
    name: '旁站记录',
    titleKey: 'boxun.bystanderRecord',
    icon: 'ep:document',
    sort: 2,
    parentId: 26
  },
  {
    id: 29,
    path: '/boxun/concrete-construction-record',
    name: '混凝土施工记录',
    titleKey: 'boxun.concreteConstructionRecord',
    icon: 'ep:document',
    sort: 3,
    parentId: 26
  },
  {
    id: 30,
    path: '/boxun/block-retention-ledger',
    name: '试块留置台账',
    titleKey: 'boxun.blockRetentionLedger',
    icon: 'ep:grid',
    sort: 4,
    parentId: 26
  },
  {
    id: 31,
    path: '/boxun/commission-order',
    name: '委托单管理',
    titleKey: 'boxun.commissionOrder',
    icon: 'ep:document',
    sort: 9
  },
  {
    id: 32,
    path: '/boxun/template-management',
    name: '我的模板',
    titleKey: 'boxun.templateManagement',
    icon: 'ep:setting',
    sort: 10
  },
  {
    id: 40,
    path: '/boxun/payment-settings',
    name: '支付设置',
    titleKey: 'boxun.paymentSettings',
    icon: 'ep:wallet',
    sort: 11
  },
  {
    id: 33,
    path: '/boxun/property-info',
    name: '楼盘信息',
    titleKey: 'boxun.propertyInfo',
    icon: 'ep:tickets',
    sort: 90,
    visible: false
  },
  {
    id: 34,
    path: '/boxun/project-company',
    name: '参建单位',
    titleKey: 'boxun.projectCompany',
    icon: 'ep:user',
    sort: 91,
    visible: false
  },
  {
    id: 35,
    path: '/boxun/commission-order-sample',
    name: '委托单样品',
    titleKey: 'boxun.commissionOrderSample',
    icon: 'ep:document',
    sort: 92,
    visible: false
  },
  {
    id: 36,
    path: '/boxun/experimental-report',
    name: '试验报告',
    titleKey: 'boxun.experimentalReport',
    icon: 'ep:document',
    sort: 93,
    visible: false
  },
  {
    id: 37,
    path: '/boxun/account-payment',
    name: '账户支付状态',
    titleKey: 'boxun.accountPayment',
    icon: 'ep:key',
    sort: 94,
    visible: false
  },
  {
    id: 38,
    path: '/boxun/specimen-evaluation',
    name: '试块评定',
    titleKey: 'boxun.specimenEvaluation',
    icon: 'ep:data-analysis',
    sort: 5,
    parentId: 26
  }
]

const parents = new Map(catalog.map((entry) => [entry.id, entry]))

export const pageMetadata = Object.freeze({
  ...Object.fromEntries(
    catalog
      .filter((entry) => entry.type !== 1)
      .map((entry, index) => {
        const parent = parents.get(entry.parentId)
        return [
          entry.path,
          Object.freeze({
            titleKey: entry.titleKey,
            icon: entry.icon,
            order: index,
            // group 保存父级路径，供菜单展开与权限分组使用；groupTitleKey 单独保存
            // 父级翻译键，供面包屑显示，避免把路径当翻译键拼出 app.boxun/generated-data。
            ...(parent ? { group: parent.path.slice(1), groupTitleKey: parent.titleKey } : {}),
            ...(entry.visible === false ? { hidden: true } : {})
          })
        ]
      })
  ),
  '/system/dict/data': Object.freeze({
    titleKey: 'system.dictData',
    icon: 'ep:grid',
    order: 100,
    group: 'system',
    groupTitleKey: 'app.system',
    hidden: true
  })
})

export function createDefaultMenus() {
  return catalog.map((entry) => {
    const parent = parents.get(entry.parentId)
    return {
      id: entry.id,
      name: entry.name,
      permission: entry.permission ?? '',
      type: entry.type ?? 2,
      sort: entry.sort,
      parentId: entry.parentId ?? 0,
      path:
        parent && entry.path.startsWith(`${parent.path}/`)
          ? entry.path.slice(parent.path.length + 1)
          : entry.path,
      icon: entry.icon,
      component: entry.type === 1 ? '' : `${entry.path.slice(1)}/index`,
      status: 1,
      visible: entry.visible ?? true,
      keepAlive: true,
      alwaysShow: true,
      createTime: created
    }
  })
}

function flattenMenus(menus, parentId = 0, visited = new Set()) {
  const rows = []
  for (const menu of menus) {
    const id = String(menu.id)
    if (visited.has(id)) {
      continue
    }
    visited.add(id)
    rows.push({ ...menu, parentId: menu.parentId ?? parentId })
    rows.push(...flattenMenus(menu.children ?? [], menu.id, visited))
  }
  return rows
}

function resolvePath(parentPath, path) {
  const value = String(path ?? '').trim()
  if (
    !value ||
    value.includes('://') ||
    value.split('/').some((part) => ['.', '..'].includes(part))
  ) {
    return ''
  }
  const absolute = value.startsWith('/') ? value : `${parentPath}/${value}`
  return absolute.replace(/\/{2,}/g, '/').replace(/\/$/, '') || '/'
}

function resolveComponentPath(value) {
  if (typeof value !== 'string') {
    return ''
  }
  const component = value.trim()
  if (
    !component ||
    component.startsWith('//') ||
    /^[a-z][a-z\d+.-]*:/i.test(component) ||
    // eslint-disable-next-line no-control-regex -- 拒绝控制字符和 URL 分隔符。
    /[\\%?#\x00-\x1f\x7f]/.test(component) ||
    component.split('/').some((part) => ['.', '..'].includes(part))
  ) {
    return ''
  }
  return `/${component.replace(/^\/+/, '')}`.replace(/\/index$/, '') || '/'
}

export function buildNavigation(menus, availablePaths) {
  const available = new Set(availablePaths)
  const children = new Map()
  for (const row of flattenMenus(menus)) {
    const parentId = String(row.parentId)
    const siblings = children.get(parentId) ?? []
    siblings.push(row)
    children.set(parentId, siblings)
  }
  const branch = (parentId, parentPath, ancestors) => {
    const siblings = [...(children.get(parentId) ?? [])]
    siblings.sort((left, right) => Number(left.sort ?? 0) - Number(right.sort ?? 0))
    return siblings.flatMap((row) => {
      const id = String(row.id)
      const enabled = row.status == null || Number(row.status) === 1
      const visible = ![false, 0, 'false', '0'].includes(row.visible)
      const type = Number(row.type ?? 2)
      if (!enabled || !visible || ![1, 2].includes(type) || ancestors.has(id)) {
        return []
      }
      const path = resolvePath(parentPath, row.path)
      if (!path) {
        return []
      }
      const nested = branch(id, path, new Set([...ancestors, id]))
      // 后端菜单路径与文件路由可能不同；叶子只映射已注册组件，目录仍保留后端层级。
      const navigationPath =
        !nested.length && !available.has(path) ? resolveComponentPath(row.component) : path
      if (!nested.length && (type === 1 || !available.has(navigationPath))) {
        return []
      }
      const item = { path: navigationPath, title: row.name, icon: row.icon || 'ep:document' }
      if (nested.length) {
        item.children = nested
      }
      return [item]
    })
  }
  return branch('0', '', new Set())
}

// 计算当前账号可访问的路由集合。目录菜单只贡献子级；叶子按组件路径或自身路径登记。
// 演示模式或缺少菜单数据时返回 null，表示不做路由裁剪。
export function collectAccessiblePaths(menus, availablePaths) {
  if (!Array.isArray(menus) || menus.length === 0) {
    return null
  }
  const available = new Set(availablePaths)
  const accessible = new Set()
  const walk = (rows, parentPath, ancestors) => {
    for (const row of rows ?? []) {
      const id = String(row.id)
      if (ancestors.has(id)) {
        continue
      }
      const enabled = row.status == null || Number(row.status) === 1
      const visible = ![false, 0, 'false', '0'].includes(row.visible)
      if (!enabled || !visible) {
        continue
      }
      const path = resolvePath(parentPath, row.path)
      const children = row.children ?? []
      const nextAncestors = new Set([...ancestors, id])
      if (children.length) {
        walk(children, path, nextAncestors)
        continue
      }
      const componentPath = resolveComponentPath(row.component)
      for (const candidate of [path, componentPath]) {
        if (candidate && available.has(candidate)) {
          accessible.add(candidate)
        }
      }
    }
  }
  walk(menus, '', new Set())
  return accessible
}

// 菜单 keepAlive 默认为 true；返回 keepAlive 显式为 false 的叶子路径，需要禁用缓存。
export function collectNoCachePaths(menus, availablePaths) {
  const available = new Set(availablePaths ?? [])
  const result = new Set()
  const walk = (rows, parentPath) => {
    for (const row of rows ?? []) {
      const type = Number(row.type ?? 2)
      const path = resolvePath(parentPath, row.path)
      const children = row.children ?? []
      if (children.length) {
        walk(children, path)
        continue
      }
      if (type !== 1 && row.keepAlive === false) {
        const componentPath = resolveComponentPath(row.component)
        for (const candidate of [path, componentPath]) {
          if (candidate && (available.size === 0 || available.has(candidate))) {
            result.add(candidate)
          }
        }
      }
    }
  }
  walk(menus, '')
  return result
}
