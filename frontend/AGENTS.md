# 管理后台模板维护约定

## 职责与真实入口

- 本目录是 CLI 复制的 Vue 应用模板，不是独立平台工作台。应用壳、路由、偏好和示例可以在新目录中独立安装构建。
- 默认配置只来自根 `app.config.json`；应用使用 `src/config.ts` 的 `appConfig`、`settings`、`resetSettings` 与 `resolveAssetUrl`。禁止将公司、账号、地址或部署目录写入源码。
- `src/main.ts` 装配 Vue、Element Plus 和路由。依赖版本以 `package.json` 与 `package-lock.json` 为准，不引用外部仓库 `node_modules` 或宿主私有页面。
- `src/layout/` 负责布局与设置；`src/pages/` 负责路由页面；`src/features/examples/` 负责示例模型、持久化和文案。

## 最小消费示例

```ts
import { appConfig, settings, resetSettings } from '@/config'

settings.title = appConfig.title
settings.dark = true
resetSettings()
```

页面的用户可见文案使用 `import { t } from '@/locales'`；功能文案必须在功能边界声明并由 `src/locales/index.ts` 装配。局部 UI 放在页面目录的 `components/` 下以避免生成路由。

## 配置、生命周期与错误

- `app.config.json` 必须保留版本和模板标识；`appId` 不得在初始化后随意改变，否则用户将进入新的浏览器存储空间。
- 页面由文件树自动生成并懒加载，导航元信息不是第二套路由事实源。页面文件名 `index.vue` 对应目录根，`[...path].vue` 是 404。
- UI 偏好通过 Vue 响应式对象自动持久化，恢复默认只覆盖本应用的偏好。存储失败明确记录到控制台并保留当前内存状态。
- 主题变量作用于 document 根节点，让 body 中的弹窗、抽屉、下拉框、加载遮罩与空态继承深色模式。禁止向具体业务页面散落全局白底补丁。
- 本地示例数据不代表服务端认证或持久化。真实业务的鉴权、API 错误与数据生命周期由宿主负责。
- 新功能优先从元数据与已有生成链路产出；不得手工修改可重复生成的业务页面来掩盖生成器缺陷。

## 验证与验收

在本目录运行：

```bash
npm ci
npm run check
npm run dev
```

首次无锁文件时先运行 `npm install` 并提交锁文件。修改 CLI 模板后还要由 CLI 生成到临时目录，在生成结果中重新安装、检查与构建，防止模板外文件依赖泄漏。

真实浏览器检查：首页、CRUD、左树右表、404、菜单折叠、关闭活动页签、设置即时生效与刷新后持久化；深色模式下打开弹窗、抽屉与下拉框，检查加载/空态/禁用态；在手机宽度检查菜单遮罩与内容可读性。没有真实浏览器证据不得宣称交互通过。

保留 `LICENSE` 与 README 来源说明，修改提取边界时同步更新。不要恢复原宿主的业务品牌、后端菜单依赖或隐式统计脚本。

## 公共系统页面接入

- 真实复制入口是 CLI 的 `src/frontend-page-packages.js`，固定消费 `lib/system/{user,foundation,file,logger}/ui/src/views` 的常用页面；只按 `index.vue` 注册路由。通过 CLI 生成到新目录后验收系统页面，模板目录本身不保存页面包副本。
- `src/page-packages/manifest.json` 在初始化时生成来源和校验和。修改这些页面要回到清单指向的公共库源码，再生成新工程；禁止修改模板外的生成副本掩盖原页面问题。
- 复制的 API 以 `src/config/axios` 为请求边界。`app.config.json` 的 `dataMode` 明确区分 demo/api，API 模式严禁失败后静默回退演示数据。
- 页面使用的宿主契约归 `src/compat`、`src/components`、`src/hooks`、`src/utils` 和 `src/types`。复用页面沿用上游 TypeScript 的两项兼容配置；宿主严格检查与页面检查都由 typecheck 脚本执行。
- 模板的经典布局尺寸、菜单和页签样式参考原宿主，业务名称、登录状态、远端地址不得随壳复制。应用名称和图片仍走 app.config.json 与可预览的本机图片选择。
