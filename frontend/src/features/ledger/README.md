# 台账业务控件

`SpecimenGroupCalculator.vue` 由商混、试块留置表单调用，使用 Topcoat
`specimen-group-numbers` 接口计算四类组数。用户点击后回填，保留手动填写能力。
父表单通过 `v-model:busy` 在计算期间阻止保存，过期请求结果不覆盖新表单。

插件内 `frontend/generated` 保留导入的页面快照，`src/pages/boxun` 为运行入口；
接入此控件时同步两处，业务计算只在后端维护。
