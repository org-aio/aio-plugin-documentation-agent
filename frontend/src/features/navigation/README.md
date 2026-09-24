# 菜单目录

`catalog.mjs` 是默认菜单与路由展示元信息的共同来源，覆盖首页、系统管理、基础设施和功能示例。页面文件仍决定实际可注册的路由；字典详情只提供隐藏路由元信息。

`createDefaultMenus()` 返回独立的默认菜单记录，供演示数据初始化。`buildNavigation(menus, availablePaths)` 使用实际菜单记录生成侧边栏，支持扁平列表和权限接口的嵌套列表；按状态、可见性、类型和排序筛选，并忽略没有注册页面的末级菜单。显示名称与图标来自菜单记录，因此菜单管理中的修改可即时反映到导航。

验证：`node --test src/features/navigation/catalog.test.mjs src/features/system-demo/model.test.mjs`。
