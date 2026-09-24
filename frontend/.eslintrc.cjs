module.exports = {
  root: true,
  env: { browser: true, node: true, es2022: true },
  extends: [
    'eslint:recommended',
    'plugin:vue/vue3-recommended',
    'plugin:@typescript-eslint/recommended',
    'prettier'
  ],
  parser: 'vue-eslint-parser',
  parserOptions: {
    parser: '@typescript-eslint/parser',
    sourceType: 'module',
    ecmaVersion: 'latest'
  },
  rules: { 'vue/multi-word-component-names': 'off' },
  // 生成页面使用 Vite 自动导入的 Vue API；与复制的公共页面一样交给类型检查和构建校验。
  ignorePatterns: [
    'dist',
    'node_modules',
    'pnpm-lock.yaml',
    'src/page-packages',
    'src/api',
    'src/pages/boxun',
    'generated'
  ]
}
