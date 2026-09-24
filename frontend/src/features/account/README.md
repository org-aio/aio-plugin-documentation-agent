# 账号会话与个人中心

宿主负责登录页、会话持久化、权限加载和 API 生命周期。个人中心界面来自 `lib/system/user/ui/src/views/Profile/PersonalCenter.vue`，通过属性注入资料、翻译和保存操作，不依赖旧宿主的 Form、Cropper 或用户 Store。

`utils/auth.ts` 是请求客户端与路由共用的低层会话入口。所有数据模式都通过现有后端 `/system/auth/login` 使用账号密码登录，只接受真实访问令牌，不提供免密入口或独立本地认证。旧版空会话、演示标记与本地认证标记均失效；旧版未声明模式的后端令牌保持兼容。密码仅用于本次登录请求，不进入浏览器存储。

`account.ts` 从 `/system/user/profile/get` 和 `/system/auth/get-permission-info` 加载真实个人资料与权限，登录完成后才进入应用。`dataMode` 仅影响业务演示数据，认证、个人资料与权限端点始终由请求边界发送到 `apiBase`，不回退本地集合。会话切换立即清空旧资料，并阻止尚未结束的旧请求覆盖新身份。退出调用 `/system/auth/logout`；退出失败保留会话并显示错误，401 表示会话已失效并清除本地状态。

个人资料更新只提交昵称、邮箱、手机、性别和头像；账号、权限等身份信息以服务端返回值为准。密码修改调用 `/system/user/profile/update-password`，当前密码和新密码不写入浏览器存储。

头像选择先在浏览器校验并返回预览和原始文件；保存资料时调用 `/infra/file/upload` 上传原始文件，取得文件服务地址后再更新资料，遵循服务端头像地址约束。服务端返回的相对头像地址按 `apiBase` 拼接，确保 `/infra/file/content/{id}` 经过 `/admin-api` 网关。

`LoginForm.vue` 提供独立账号密码表单，`showTenant` 默认关闭，`rememberAccount` 默认允许展示“记住账号”；浏览器仅按应用保存用户名，不保存密码或验证码证明。页面布局负责品牌与标题。验证码重试和取消保留表单输入，实际登录尝试结束后清空密码。

登录页读取 `/system/captcha/config` 的服务端开关和 `blockPuzzle`／`clickWord` 类型；配置读取失败时显示重试并阻止登录，不接受本地安全开关。字段校验通过后自动弹出 `CaptchaChallenge.vue`。`captcha.mjs` 复用原 Anji 验证码的 crypto-js AES-ECB/PKCS7 协议，检查与登录证明使用同一份坐标明文。`/system/captcha/get`、`/system/captcha/check` 保留原始 `repCode` 信封，所有验证码请求始终走真实 API。

挑战过期、检查失败后需获取新挑战；关闭、刷新和卸载会取消请求，并拒绝迟到响应。滑块支持鼠标、触摸和方向键，点选按 310×155 原图换算手机坐标，也可用方向键及空格选择。crypto-js 是兼容已有服务端 ECB 协议所需的唯一额外运行依赖，不复制旧验证码组件的图标和样式依赖。

验证：在模板目录运行 `node --test src/features/account/*.test.mjs`；从 CLI 生成完整工程后运行 `npm run check`，并验收验证码失败／刷新／取消、两种验证码、记住账号、修改资料、头像预览、退出、刷新与重新登录。
