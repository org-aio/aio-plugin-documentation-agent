# 系统页面演示数据

此功能为提取后的系统管理与基础设施页面提供本地演示数据。`app.config.json` 的 `dataMode` 必须显式选择 `demo` 或 `api`。演示数据保存在 `${appId}:system-demo:v1`，真实 API 的会话保存在 `${appId}:session`，二者互不替代。

- `seeds.mjs` 提供中立的部门、用户、角色、菜单、字典和日志示例。
- `model.mjs` 负责演示请求、筛选、分页、关联校验和持久化。失败写入不返回成功，未知操作报错。角色分配、权限与密码重置仅展示操作流程；密码只记录修改时间，不保存输入值。
- 文件保存在浏览器中，总量限制 2 MB。返回浏览器 Blob 地址供预览或下载，刷新后从存储重建。配置测试只测试浏览器本地存储，不连接远程服务；演示模式拒绝保存存储凭据。
- 演示导出是真实 CSV，调用方应按 MIME 使用 `.csv` 扩展名。Excel 导入与模板下载不提供伪造结果，必须连接 API 后使用。
- `config/axios/transport.mjs` 是独立可测试的 Fetch 适配器。API 模式使用显式 `apiBase`、`accessToken` 与 `tenantId`，检查 HTTP 和业务 `code`，失败不会回退到演示数据。上传保留 `{code,data}`，其他 JSON 请求返回 `data`。

页面必须展示导出的 `DEMO_NOTICE`。演示数据不能作为真实认证、鉴权或服务端持久化的证据。

验证：`node --test src/features/system-demo/model.test.mjs src/config/axios/transport.test.mjs`。
