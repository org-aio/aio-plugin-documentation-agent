<template>
  <div class="payment-settings">
    <ContentWrap>
      <div class="section-head">
        <div>
          <h2>支付设置</h2>
          <p>维护账号价目表与支付宝支付参数；保存后写入系统参数，供账号支付与注册流程使用。</p>
        </div>
        <el-button :loading="loading" type="primary" @click="handleSave">
          <Icon class="mr-5px" icon="ep:check" />保存
        </el-button>
      </div>

      <el-form label-position="top" :model="form">
        <el-divider content-position="left">账号价目表</el-divider>
        <el-table :data="form.priceList" border>
          <el-table-column label="账户类型" width="140">
            <template #default="scope">
              <el-input-number v-model="scope.row.accountType" class="!w-100%" :min="0" controls-position="right" />
            </template>
          </el-table-column>
          <el-table-column label="账户价格（元）" min-width="160">
            <template #default="scope">
              <el-input-number v-model="scope.row.accountPrice" class="!w-100%" :min="0" :precision="2" controls-position="right" />
            </template>
          </el-table-column>
          <el-table-column label="持续天数" min-width="140">
            <template #default="scope">
              <el-input-number v-model="scope.row.durationDays" class="!w-100%" :min="1" controls-position="right" />
            </template>
          </el-table-column>
          <el-table-column align="center" label="操作" width="90">
            <template #default="scope">
              <el-button link type="danger" @click="removePrice(scope.$index)">删除</el-button>
            </template>
          </el-table-column>
        </el-table>
        <el-button class="mt-12px" plain type="primary" @click="addPrice">
          <Icon class="mr-5px" icon="ep:plus" />新增档位
        </el-button>

        <el-divider content-position="left">支付宝参数</el-divider>
        <el-row class="!mx-0" :gutter="16">
          <el-col :span="12">
            <el-form-item label="启用支付宝">
              <el-switch v-model="form.alipayEnabled" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="应用 ID（appId）">
              <el-input v-model="form.alipayAppId" placeholder="请输入支付宝应用 appId" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="网关地址">
              <el-input v-model="form.alipayGateway" placeholder="例如 https://openapi.alipay.com/gateway.do" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="回调通知地址">
              <el-input v-model="form.alipayNotifyUrl" placeholder="请输入支付结果通知地址" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="应用私钥">
              <el-input v-model="form.alipayPrivateKey" type="textarea" :rows="3" placeholder="请输入应用私钥" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="支付宝公钥">
              <el-input v-model="form.alipayPublicKey" type="textarea" :rows="3" placeholder="请输入支付宝公钥" />
            </el-form-item>
          </el-col>
        </el-row>
      </el-form>
    </ContentWrap>
  </div>
</template>

<script setup lang="ts">
import * as BoxunPaymentSettingsApi from './BoxunPaymentSettingsApi'

defineOptions({ name: 'BoxunPaymentSettings' })

const message = useMessage()
const loading = ref(false)
const form = ref<BoxunPaymentSettingsApi.PaymentSettingsForm>(
  BoxunPaymentSettingsApi.emptyPaymentSettings()
)
/** 配置键 → infra_config 记录 id；新建后回填。 */
const configIds = ref<Record<string, string | number>>({})

const addPrice = () => {
  form.value.priceList.push({ accountType: 0, accountPrice: 0, durationDays: 30 })
}

const removePrice = (index: number) => {
  form.value.priceList.splice(index, 1)
}

/** 读取全部 boxun.pay.* 配置并映射到表单。 */
const load = async () => {
  loading.value = true
  try {
    const data = await BoxunPaymentSettingsApi.getConfigPage({ pageNo: 1, pageSize: 100, category: 'boxun-pay' })
    const settings = BoxunPaymentSettingsApi.emptyPaymentSettings()
    const ids: Record<string, string | number> = {}
    for (const row of data.list) {
      const key = String(row.key)
      ids[key] = row.id
      switch (key) {
        case BoxunPaymentSettingsApi.PAYMENT_CONFIG_KEYS.alipayEnabled:
          settings.alipayEnabled = String(row.value) === 'true'
          break
        case BoxunPaymentSettingsApi.PAYMENT_CONFIG_KEYS.alipayAppId:
          settings.alipayAppId = String(row.value ?? '')
          break
        case BoxunPaymentSettingsApi.PAYMENT_CONFIG_KEYS.alipayPrivateKey:
          settings.alipayPrivateKey = String(row.value ?? '')
          break
        case BoxunPaymentSettingsApi.PAYMENT_CONFIG_KEYS.alipayPublicKey:
          settings.alipayPublicKey = String(row.value ?? '')
          break
        case BoxunPaymentSettingsApi.PAYMENT_CONFIG_KEYS.alipayGateway:
          settings.alipayGateway = String(row.value ?? '')
          break
        case BoxunPaymentSettingsApi.PAYMENT_CONFIG_KEYS.alipayNotifyUrl:
          settings.alipayNotifyUrl = String(row.value ?? '')
          break
        case BoxunPaymentSettingsApi.PAYMENT_CONFIG_KEYS.priceList:
          settings.priceList = parsePriceList(row.value)
          break
      }
    }
    configIds.value = ids
    form.value = settings
  } finally {
    loading.value = false
  }
}

const parsePriceList = (value: unknown): BoxunPaymentSettingsApi.AccountPrice[] => {
  try {
    const parsed = JSON.parse(String(value ?? '[]'))
    if (!Array.isArray(parsed)) return BoxunPaymentSettingsApi.emptyPaymentSettings().priceList
    return parsed.map((item) => ({
      accountType: Number(item.accountType ?? 0),
      accountPrice: Number(item.accountPrice ?? 0),
      durationDays: Number(item.durationDays ?? 0)
    }))
  } catch {
    return BoxunPaymentSettingsApi.emptyPaymentSettings().priceList
  }
}

const saveConfig = async (key: string, name: string, value: string) => {
  const id = configIds.value[key]
  const payload = {
    category: 'boxun-pay',
    name,
    key,
    value,
    type: 2,
    visible: true,
    remark: '支付设置'
  }
  if (id === undefined) {
    const created = await BoxunPaymentSettingsApi.createConfig(payload)
    configIds.value[key] = created.id
  } else {
    await BoxunPaymentSettingsApi.updateConfig({ ...payload, id })
  }
}

const handleSave = async () => {
  loading.value = true
  try {
    const keys = BoxunPaymentSettingsApi.PAYMENT_CONFIG_KEYS
    await saveConfig(keys.alipayEnabled, '启用支付宝', String(form.value.alipayEnabled))
    await saveConfig(keys.alipayAppId, '支付宝应用ID', form.value.alipayAppId)
    await saveConfig(keys.alipayPrivateKey, '支付宝应用私钥', form.value.alipayPrivateKey)
    await saveConfig(keys.alipayPublicKey, '支付宝公钥', form.value.alipayPublicKey)
    await saveConfig(keys.alipayGateway, '支付宝网关地址', form.value.alipayGateway)
    await saveConfig(keys.alipayNotifyUrl, '支付宝回调通知地址', form.value.alipayNotifyUrl)
    await saveConfig(keys.priceList, '账号价目表', JSON.stringify(form.value.priceList))
    message.success('支付设置已保存')
    await load()
  } finally {
    loading.value = false
  }
}

onMounted(load)
</script>

<style scoped>
.payment-settings {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.section-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  flex-wrap: wrap;
}
.section-head h2 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}
.section-head p {
  margin: 4px 0 0;
  color: var(--el-text-color-secondary);
  font-size: 13px;
}
</style>
