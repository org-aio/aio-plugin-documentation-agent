# 页面宿主兼容层

本层供 CLI 复制的系统页面调用，保留原页面的组件、工具与自动导入契约，不提供第二套应用壳。`src/main.ts` 调用 `installPageHost(app)` 注册公共组件与权限指令；Element Plus 由应用入口安装，图标只使用本地 `@element-plus/icons-vue`。

`useI18n` 的旧页面文案位于本目录 `messages.ts`，其他键交给宿主 `src/locales`。`src/types/auto-imports.d.ts` 与 Vite 自动导入配置保持一致，类型检查无需先启动 Vite。

`dataMode: demo` 允许页面演示操作；`api` 模式读取本应用 `${appConfig.appId}:session` 本地会话对象的 `accessToken`、`tenantId`、`permissions` 与 `roles`。权限缺失时拒绝操作；后端仍须执行真正的鉴权。会话变更后刷新页面。页面缓存使用独立的 `page-cache` 键，不会因刷新菜单缓存删除登录会话。

字典枚举与演示种子共用中立的固定值，宿主可通过 `setDictOptions` 写入服务端字典。弹窗、标签与提示统一使用 Element Plus 主题变量，继承根节点的深色模式。

在模板根目录执行 `npm run typecheck`、`npm run lint`；在 CLI 生成结果中执行 `npm run check` 并检查真实页面交互。
