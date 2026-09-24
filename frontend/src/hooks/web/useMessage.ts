import { ElMessage, ElMessageBox } from 'element-plus'
import { useI18n } from './useI18n'

export const useMessage = () => {
  const { t } = useI18n()
  const buttons = {
    confirmButtonText: t('common.ok'),
    cancelButtonText: t('common.cancel')
  }
  const confirm = (content: string, title = t('common.confirmTitle')) =>
    ElMessageBox.confirm(content, title, { ...buttons, type: 'warning' })

  return {
    info: ElMessage.info,
    error: ElMessage.error,
    success: ElMessage.success,
    warning: ElMessage.warning,
    alert: (content: string, title = t('common.reminder')) =>
      ElMessageBox.alert(content, title, buttons),
    confirm,
    delConfirm: (content = t('common.delMessage'), title?: string) => confirm(content, title),
    exportConfirm: (content = t('common.exportMessage'), title?: string) => confirm(content, title),
    prompt: (content: string, title = t('common.reminder')) =>
      ElMessageBox.prompt(content, title, buttons)
  }
}
