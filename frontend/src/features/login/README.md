# 登录页展示

`LoginPresentation.vue` 负责布局与品牌展示，`LoginSettings.vue` 提供配置编辑器。真实账号登录与验证码交互由 `features/account/LoginForm.vue` 负责，页面只组合这两个组件。

## 配置与预览

应用的 `app.config.json` 中 `login` 是默认配置，运行时通过 `settings.login` 读取和修改，沿用应用的本地偏好保存机制。登录页右上角设置按钮直接打开登录页设置；进入应用后，可从设置抽屉的「登录页」页签编辑并打开预览。预览使用禁用的示例表单，不创建登录表单实例。

| 字段                                        | 作用                                           |
| ------------------------------------------- | ---------------------------------------------- |
| `layout`、`panelPosition`                   | 左右分栏或居中卡片，以及分栏时的表单位置       |
| `backgroundType`、`backgroundColor`         | 渐变、纯色或图片背景及其底色                   |
| `backgroundImage`、`backgroundOverlay`      | 背景图片与 0–1 的暗色遮罩强度                  |
| `heroTitle`、`heroDescription`、`heroImage` | 欢迎文案和配图；分栏未配图片时显示默认中性插画 |
| `formTitle`、`formDescription`              | 登录表单的标题和说明                           |
| `showBrand`、`showHero`                     | 显示应用标识、欢迎区域                         |
| `showTenant`、`rememberAccount`             | 显示租户输入和记住账号选项                     |
| `showThemeToggle`                           | 显示主题切换按钮                               |
| `showFooter`、`footer`                      | 显示独立的登录页脚                             |

默认值、类型和字段校验统一在 `config.mjs` 与 `config.d.mts` 中定义。修改编辑器应同时遵守该契约。「恢复登录页默认」仅恢复当前应用的登录页默认值；抽屉底部的恢复默认按钮恢复整个应用设置。

「导出配置」下载归一化后的平铺登录展示 JSON，可作为 CLI 的 `--login-config` 文件复用。同源本地图片会按部署前缀读取并转为 data URL，换目录、换应用时无需复制图片；已有 data URL 和 HTTP(S) 图片地址保持不变。导出兼容 CLI 的单张 5 MB 图片上限；图片不可访问、格式不支持或过大时会显示错误，不下载残缺配置。导出内容不包含账号、密码、令牌、验证码状态或验证码安全设置。

## 图片与扩展插槽

背景图片和欢迎配图复用应用图标的文件校验：支持 SVG、PNG、JPEG、WebP、ICO，单张最大 1 MB，成功解码后才替换当前图片。上传内容以 data URL 保存；项目中的资源路径和 HTTP(S) 图片地址也可在配置中声明，项目相对路径自动使用部署前缀。

`LoginPresentation` 接受 `configuration` 和可选的 `embedded` 属性，暴露以下插槽：

| 插槽      | 用途                                         |
| --------- | -------------------------------------------- |
| 默认插槽  | 放置真实登录表单或预览表单                   |
| `hero`    | 替换欢迎区域正文与配图，接收 `configuration` |
| `footer`  | 替换页脚内容，接收 `configuration`           |
| `actions` | 在主题按钮旁添加页面操作，例如设置入口       |

组件支持浅色、深色、移动端布局和减少动态效果偏好。嵌入预览会禁用主题切换按钮，避免预览操作修改全局主题。

## 源码维护

源头为 `packages/cli/templates/admin-shell`。请在模板中修改，再通过 CLI 重新生成应用，不直接补丁生成目录。验证包括模板 ESLint/Prettier、生成应用的完整类型检查，以及登录页和设置预览的浏览器工作流。
