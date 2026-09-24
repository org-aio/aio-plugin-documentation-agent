# 个人中心组件

`PersonalCenter.vue` 是可直接被 Vue 3 + Element Plus 宿主消费的个人资料编辑器，与旧 `Index.vue` 的 Form、Cropper、用户 Store 依赖独立。

宿主传入 `profile`、`demo`、翻译函数 `t`，以及 `readAvatar(file)`、`saveProfile(changes)`、`changePassword(oldPassword, newPassword)` 三个异步操作。`readAvatar(file)` 返回预览地址和原始文件，`saveProfile` 接收昵称、邮箱、手机、性别、头像地址及可选原始文件；成功后宿主应重新读取并替换 `profile`，让摘要和表单一起更新。失败操作必须抛出错误，由组件展示原因。演示模式没有密码输入框，也不会调用密码 API。

实际装配示例位于 `packages/cli/templates/admin-shell/src/pages/user/profile.vue`，请求与会话生命周期位于同模板 `src/features/account/account.ts`。图片读取复用模板的 `src/layout/logoFile.ts`，限制格式与 1 MB 大小并完成浏览器解码校验。所有依赖由宿主注入，页面包不启动独立服务。

验证：在仓库根执行 `node --test packages/cli/test/frontend.test.js`；由 CLI 生成新工程后在生成目录执行 `npm run check`，再通过浏览器验证资料持久化、头像预览、暗色主题和移动端布局。
