# 资料员Agent

`资料员Agent` 是 AIO v2 `process` 全栈插件，后端由 Topcoat 实现。它把原 `boxun-app` 的项目资料、业务台账、生成记录和系统管理能力装进一个可由租户安装即用的插件。

插件使用 AIO 提供的租户隔离 PostgreSQL，前端通过 `window.aioPlugin` 调用后端，不接触数据库凭据或宿主会话 Cookie。首次进入时前端会使用宿主验证过的用户与租户上下文自动建立会话。

## 功能

- 项目资料：项目信息、项目单位、楼盘信息；
- 业务台账：商混台账、原材料进场台账、试块留置台账、委托单与样品；
- 记录与报告：见证记录、旁站记录、混凝土施工记录、试验报告、历史天气；
- 资料生成：模板继承、字段映射、试件组数计算、试块强度评定、套件和委托单下载；
- 系统管理：用户、角色、菜单、部门、岗位、字典、参数、邮件账户和租户内文件。

## 运行边界

- 历史天气同步通过 AIO process 出站代理访问清单声明的 `https://tianqi.2345.com/Pc/GetHistory`，解析每日天气并幂等写入租户数据库。
- 邮件发送使用 `system_mail_account` 中的 SMTP 账号，并校验委托单对应的送检单位邮箱；未配置或发送失败时返回可诊断错误。
- Excel 导入使用 `.xlsx/.xls` 文件解析；导出和生成物使用真实 `.xlsx` 工作簿，经 AIO 前端桥以 base64 信封传输并在浏览器还原为 Blob。

## 构建

```bash
sh scripts/build.sh
```

脚本先在 `frontend/` 安装锁定依赖并构建 Vue 管理端，再用 Topcoat 后端构建 Linux x86_64 ELF 到 `dist/boxun-topcoat-server`，前端静态资源位于 `dist/frontend`。AIO 宿主安装时执行 `backend-topcoat/migrations` 中的受控 PostgreSQL 迁移。

## 接口

- `GET /health`、`GET /aio/describe`：进程健康与 AIO 页面定义；
- `/admin-api/system/**`：认证、权限、用户、角色、菜单、部门、岗位、字典和邮件账户；
- `/admin-api/infra/**`：参数配置和文件上传/下载；
- `/admin-api/boxun/**`：项目、台账、委托单、记录、天气、模板和资料生成。

所有业务接口要求由 AIO 宿主注入 `x-aio-token`、`x-aio-tenant-id` 和 `x-aio-user-id`；宿主自动登录只接受这些已验证头，不接受浏览器自报身份。
