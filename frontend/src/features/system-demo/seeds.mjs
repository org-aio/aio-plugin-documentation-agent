import { createDefaultMenus, MENU_CATALOG_VERSION } from '../navigation/catalog.mjs'

const created = '2026-01-15 09:30:00'

export const DICTIONARIES = Object.freeze({
  common_status: [
    ['启用', '1', 'success'],
    ['停用', '0', 'danger']
  ],
  system_user_sex: [
    ['未知', '0', 'info'],
    ['男', '1', 'primary'],
    ['女', '2', 'warning']
  ],
  system_menu_type: [
    ['目录', '1', 'primary'],
    ['菜单', '2', 'success'],
    ['按钮', '3', 'warning']
  ],
  system_role_type: [
    ['内置', '1', 'danger'],
    ['自定义', '2', 'primary']
  ],
  system_data_scope: [
    ['全部数据', '1', 'success'],
    ['指定部门', '2', 'warning'],
    ['本部门', '3', 'primary'],
    ['本部门及以下', '4', 'primary'],
    ['仅本人', '5', 'info']
  ],
  system_login_type: [
    ['账号登录', '100', 'primary'],
    ['社交登录', '101', 'info'],
    ['短信登录', '103', 'info'],
    ['主动登出', '200', 'primary'],
    ['强制登出', '202', 'danger']
  ],
  system_login_result: [
    ['成功', '0', 'success'],
    ['账号或密码不正确', '10', 'danger'],
    ['用户被禁用', '20', 'warning'],
    ['验证码不存在', '30', 'info'],
    ['验证码不正确', '31', 'info'],
    ['未知异常', '100', 'danger']
  ],
  user_type: [
    ['会员', '1', 'primary'],
    ['管理员', '2', 'success']
  ],
  infra_boolean_string: [
    ['是', 'true', 'success'],
    ['否', 'false', 'info']
  ],
  infra_config_type: [
    ['系统内置', '1', 'warning'],
    ['自定义', '2', 'primary']
  ],
  infra_file_storage: [
    ['数据库', '1', 'primary'],
    ['本地存储', '10', 'success'],
    ['FTP 服务器', '11', 'info'],
    ['SFTP 服务器', '12', 'info'],
    ['S3 对象存储', '20', 'info']
  ],
  infra_operate_type: [
    ['其他', '0', 'info'],
    ['查询', '1', 'primary'],
    ['新增', '2', 'success'],
    ['修改', '3', 'warning'],
    ['删除', '4', 'danger'],
    ['导出', '5', 'info'],
    ['导入', '6', 'info']
  ],
  infra_api_error_log_process_status: [
    ['未处理', '0', 'danger'],
    ['已处理', '1', 'success'],
    ['已忽略', '2', 'info']
  ]
})

const dictionaryRows = Object.entries(DICTIONARIES).flatMap(([dictType, items], groupIndex) =>
  items.map(([label, value, colorType], index) => ({
    id: groupIndex * 10 + index + 1,
    sort: index + 1,
    label,
    value,
    dictType,
    status: 1,
    colorType,
    cssClass: '',
    remark: '',
    createTime: created
  }))
)

export const createSeedState = () => ({
  version: 1,
  menuCatalogVersion: MENU_CATALOG_VERSION,
  nextId: 100,
  users: [
    {
      id: 1,
      username: 'admin',
      nickname: '演示管理员',
      deptId: 1,
      postIds: [1],
      email: 'admin@example.invalid',
      mobile: '13800000000',
      sex: 0,
      avatar: '',
      loginIp: '127.0.0.1',
      status: 1,
      remark: '本地演示账号',
      loginDate: created,
      createTime: created,
      passwordChangedAt: null
    },
    {
      id: 2,
      username: 'operator',
      nickname: '演示操作员',
      deptId: 2,
      postIds: [2],
      email: 'operator@example.invalid',
      mobile: '13900000000',
      sex: 0,
      avatar: '',
      loginIp: '127.0.0.1',
      status: 1,
      remark: '',
      loginDate: created,
      createTime: created,
      passwordChangedAt: null
    }
  ],
  roles: [
    {
      id: 1,
      name: '管理员',
      code: 'admin',
      sort: 1,
      status: 1,
      type: 2,
      dataScope: 1,
      dataScopeDeptIds: [],
      createTime: created
    },
    {
      id: 2,
      name: '操作员',
      code: 'operator',
      sort: 2,
      status: 1,
      type: 2,
      dataScope: 2,
      dataScopeDeptIds: [2],
      createTime: created
    }
  ],
  userRoles: { 1: [1], 2: [2] },
  roleMenus: { 1: createDefaultMenus().map((menu) => menu.id), 2: [1, 2, 4] },
  menus: createDefaultMenus(),
  depts: [
    {
      id: 1,
      name: '演示组织',
      parentId: 0,
      status: 1,
      sort: 1,
      leaderUserId: 1,
      phone: '',
      email: 'contact@example.invalid',
      createTime: created
    },
    {
      id: 2,
      name: '运营组',
      parentId: 1,
      status: 1,
      sort: 1,
      leaderUserId: 2,
      phone: '',
      email: 'operations@example.invalid',
      createTime: created
    }
  ],
  posts: [
    { id: 1, name: '管理员', code: 'admin', sort: 1, status: 1, remark: '', createTime: created },
    {
      id: 2,
      name: '运营专员',
      code: 'operator',
      sort: 2,
      status: 1,
      remark: '',
      createTime: created
    }
  ],
  dictTypes: Object.keys(DICTIONARIES).map((type, index) => ({
    id: index + 1,
    name: type.replaceAll('_', ' '),
    type,
    status: 1,
    remark: '演示字典',
    createTime: created
  })),
  dictData: dictionaryRows.map((row) => ({ ...row })),
  configs: [
    {
      id: 1,
      category: 'ui',
      name: '默认页大小',
      key: 'demo.page-size',
      value: '10',
      type: 2,
      visible: true,
      remark: '仅用于本地演示',
      createTime: created
    }
  ],
  files: [
    {
      id: 1,
      configId: 1,
      name: 'readme.txt',
      path: 'demo/readme.txt',
      url: 'data:text/plain;charset=utf-8,%E6%9C%AC%E5%9C%B0%E6%BC%94%E7%A4%BA%E6%96%87%E4%BB%B6',
      size: 24,
      type: 'text/plain',
      createTime: created
    }
  ],
  fileConfigs: [
    {
      id: 1,
      name: '浏览器本地存储',
      storage: 10,
      master: true,
      visible: true,
      config: { basePath: 'demo-files', domain: '' },
      remark: '仅保存到当前浏览器',
      createTime: created
    }
  ],
  loginLogs: [
    {
      id: 1,
      logType: 100,
      traceId: 'demo-login-1',
      userId: 1,
      userType: 2,
      username: 'admin',
      result: 0,
      status: 0,
      userIp: '127.0.0.1',
      userAgent: 'Demo browser',
      createTime: created
    }
  ],
  operateLogs: [
    {
      id: 1,
      traceId: 'demo-operation-1',
      userType: 2,
      userId: 1,
      userName: '演示管理员',
      type: '查询',
      subType: '列表',
      bizId: 1,
      action: '查看演示数据',
      extra: '',
      requestMethod: 'GET',
      requestUrl: '/system/user/page',
      userIp: '127.0.0.1',
      userAgent: 'Demo browser',
      creator: 'admin',
      creatorName: '演示管理员',
      createTime: created
    }
  ],
  apiAccessLogs: [
    {
      id: 1,
      traceId: 'demo-api-1',
      userId: 1,
      userType: 2,
      applicationName: 'admin-demo',
      requestMethod: 'GET',
      requestParams: '{}',
      responseBody: '{"code":0}',
      requestUrl: '/system/user/page',
      userIp: '127.0.0.1',
      userAgent: 'Demo browser',
      operateModule: '系统管理',
      operateName: '用户列表',
      operateType: 1,
      beginTime: created,
      endTime: created,
      duration: 18,
      resultCode: 0,
      resultMsg: '成功',
      createTime: created
    }
  ],
  apiErrorLogs: [
    {
      id: 1,
      traceId: 'demo-error-1',
      userId: 2,
      userType: 2,
      applicationName: 'admin-demo',
      requestMethod: 'GET',
      requestParams: '{}',
      requestUrl: '/demo/unavailable',
      userIp: '127.0.0.1',
      userAgent: 'Demo browser',
      exceptionTime: created,
      exceptionName: 'DemoRequestError',
      exceptionMessage: '演示错误日志',
      exceptionRootCauseMessage: '用于展示处理流程',
      exceptionStackTrace: 'DemoRequestError: example',
      exceptionClassName: 'DemoRequestError',
      exceptionFileName: 'demo',
      exceptionMethodName: 'request',
      exceptionLineNumber: 1,
      processUserId: 0,
      processStatus: 0,
      processTime: null,
      resultCode: 500,
      createTime: created
    }
  ]
})
