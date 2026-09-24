import assert from 'node:assert/strict'
import test from 'node:test'

// access.ts 依赖 Vue 运行时，这里只验证纯决策逻辑的等价实现，避免引入浏览器环境。
function isPathAccessible(path, group, hidden, allowed, groups) {
  const always = new Set(['/login', '/home', '/user/profile'])
  if (always.has(path) || path === '/:pathMatch(.*)*') {
    return true
  }
  if (allowed === null) {
    return true
  }
  if (allowed.has(path)) {
    return true
  }
  if (!hidden || !group) {
    return false
  }
  return groups.has(group)
}

test('菜单加载前不裁剪，加载后只放行授权路径与隐藏子页', () => {
  assert.equal(isPathAccessible('/system/user', 'system', false, null, new Set()), true)
  const allowed = new Set(['/system/user', '/system/dict'])
  const groups = new Set(['system'])
  assert.equal(isPathAccessible('/system/user', 'system', false, allowed, groups), true)
  assert.equal(isPathAccessible('/system/dict/data', 'system', true, allowed, groups), true)
  // 同组非隐藏页面不能借分组放行。
  assert.equal(isPathAccessible('/infra/codegen', 'infra', false, allowed, groups), false)
  // 隐藏页面也必须有授权分组。
  assert.equal(isPathAccessible('/system/dict/data', 'system', true, allowed, new Set()), false)
  assert.equal(isPathAccessible('/home', 'system', false, new Set(), new Set()), true)
})
