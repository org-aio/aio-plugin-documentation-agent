# 应用管理后台

Vue 3 与 Element Plus 管理后台。经典暗侧栏、工具条、面包屑、多页签、响应式布局和个性化设置开箱即用。示例数据保存在当前浏览器，可在没有后端服务的情况下体验 CRUD 和左树右表。

## 开发与验证

需要 Node.js 22.12 或更新版本。在本目录执行：

```bash
npm ci
npm run dev
```

首次从模板开发且尚无锁文件时使用 `npm install`，随后提交生成的 `package-lock.json`。访问终端打印的本机地址。

```bash
npm run check
npm run preview
```

`check` 依次执行类型检查、示例模型测试、ESLint、格式检查和生产构建。`dist/` 是可部署静态资源；采用 hash 路由，不要求服务器为页面路径配置回退。

## 应用配置

根目录 `app.config.json` 是应用默认值的唯一入口。`title`、`logo`、`themeColor`、页签与页脚可在右上角设置中覆盖，偏好按 `appId` 隔离保存在当前浏览器；恢复默认会重新使用根配置。

| 字段                                     | 用途                                 |
| ---------------------------------------- | ------------------------------------ |
| `formatVersion` / `template`             | 配置版本 `1` 与模板标识 `yudao`      |
| `appId`                                  | 应用标识，也是本地存储隔离依据       |
| `title` / `logo`                         | 应用名称与标志；标志同时用于 favicon |
| `themeColor`                             | 六位十六进制主题色，如 `#409eff`     |
| `locale`                                 | 当前支持 `zh-CN`                     |
| `basePath`                               | 静态部署前缀，如 `/` 或 `/admin/`    |
| `apiBase`                                | 后续接入真实后端时使用的 API 基址    |
| `showTagsView` / `showFooter` / `footer` | 页签、页脚开关与页脚文字             |

默认图标为 `public/logo.svg`。本地资源路径自动跟随 `basePath`；也支持 HTTPS 图片地址。改动部署前缀后重新构建。配置中不得写入密钥或账号；此文件会进入浏览器资源。

## 页面与功能边界

`src/pages/**/*.vue` 自动生成懒加载路由，`home.vue` 对应 `/home`，`[...path].vue` 处理 404。根路径重定向到首页。页面局部组件放入 `components/`，该目录不生成路由。

`src/router.ts` 的 `pageMetadata` 仅声明标题、图标与排序；没有元信息的新页面仍会生成路由和菜单。页面文案通过 `src/locales/index.ts` 的 `t(key)` 读取。示例文案和数据规则集中于 `src/features/examples/`，接入真实后端时在该功能边界替换数据来源。

本模板不包含认证服务。接入真实业务前，应由宿主配置登录和服务端授权，不能把示例界面当作权限边界。

维护和消费约定见 [AGENTS.md](./AGENTS.md)。

## 来源与许可

布局提取自本仓库现有 Yudao Vue 管理后台，保留经典布局的侧栏 / Logo / 工具条 / 多页签 / 内容结构及语义主题变量。来源文件相对于 Studio 仓库：

- `apps/iot-app/ui/src/layout/components/useRenderLayout.tsx` 的 `renderClassic`
- `apps/iot-app/ui/src/layout/components/Logo/src/Logo.vue`
- `apps/iot-app/ui/src/layout/components/Setting/src/Setting.vue`
- `apps/iot-app/ui/src/styles/var.css`
- `apps/iot-app/ui/LICENSE`

提取版本移除了原宿主的业务菜单、后端权限与账户状态、IoT 注册、外部页面包和统计脚本依赖；改为文件式路由和应用配置。首页中性图形为本模板新增的 CSS / SVG 资源。

原始 MIT 版权声明 `Copyright (c) 2021-present Archer` 和完整许可见 [LICENSE](./LICENSE)。分发或再次提取本模板时必须保留该许可。依赖包分别遵循各自许可证。

## 系统管理与基础设施

CLI 会从平台的 `lib/system/{user,foundation,file,logger}/ui` 复制系统页面包到 `src/page-packages/`，从平台当前 API 入口复制对应客户端到 `src/api/`。初始化清单 `src/page-packages/manifest.json` 记录每个文件的原始路径和 SHA-256；模板不维护另一份系统页面。生成后的工程可独立安装和构建，无需引用平台的 node_modules。

默认菜单包含用户、角色、菜单、部门、岗位、字典、登录日志、操作日志，以及参数配置、文件管理、文件配置、API 访问日志和 API 错误日志。只将页面包中的 `index.vue` 注册为路由，表单和详情作为局部组件使用。字典数据兼容原页面的跳转地址。

`app.config.json` 的 `dataMode` 默认为 `demo`。系统页面顶部持续显示演示提示；演示数据保存在当前浏览器，重新打开页面仍保留。`src/features/system-demo/` 负责演示数据，`src/config/axios/` 是可切换的请求适配层。`dataMode: "api"` 时请求 `apiBase`，服务端失败会显示错误，不会回退演示数据。宿主仍须接入登录、会话和服务端权限；模板不提供认证后端。

页面包保持上游文件原样，排除宿主格式化与 ESLint 重写；`tsconfig.pages.json` 使用上游的 `noImplicitAny: false` 和 `strictFunctionTypes: false` 检查页面及客户端，宿主本身继续使用严格类型检查。两部分均包含在 `npm run check` 中。兼容组件、自动导入声明和宿主工具分别位于 `src/components/`、`src/types/` 和 `src/utils/`。
