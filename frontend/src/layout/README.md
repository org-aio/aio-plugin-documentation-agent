# 应用壳

`AdminLayout.vue` 提供经典暗侧栏、工具条、面包屑、多页签和响应式内容区，顶部工具条提供设置入口。`Logo.vue` 从应用设置读取名称与图标；`SettingsDrawer.vue` 修改共享偏好。主题变量集中于 `../styles/theme.css`。

`LogoPicker.vue` 支持选择本机 SVG、PNG、JPEG、WebP、ICO 图标并预览、恢复默认；`logoFile.ts` 负责 1 MB 大小限制、读取和实际图片解码，校验失败保留原图。图片使用 data URL 随个人偏好保存，刷新后继续可用。`AppIcon.vue` 为左上图标与设置预览共用默认图标回退；`config.ts` 的 `settingsSaveState` 将存储失败展示为当前页面临时生效。

页面可使用 `inject<() => void>('openSettings')` 打开设置。导航元数据由 `../router.ts` 随页面文件装配，布局不拥有任何业务菜单或后端账号依赖。
