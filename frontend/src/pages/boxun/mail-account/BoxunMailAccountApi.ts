// 邮件配置为系统设置页面，放在手写目录以避免后续 Studio 前端生成覆盖。
import request from '@/config/axios'

export type EntityId = string | number
export type EntityInput = Record<string, any>
export type EntityRecord = EntityInput & { id: EntityId }

export interface PageResult<T> {
  list: T[]
  total: number
}

export const getBoxunMailAccountPage = (params: Record<string, unknown>) =>
  request.get<PageResult<EntityRecord>>({ url: '/system/mail-account/page', params })

export const getBoxunMailAccount = (id: EntityId) =>
  request.get<EntityRecord>({ url: '/system/mail-account/get?id=' + id })

export const createBoxunMailAccount = (data: EntityInput) =>
  request.post<EntityRecord>({ url: '/system/mail-account/create', data })

export const updateBoxunMailAccount = (data: EntityInput) =>
  request.put<number>({ url: '/system/mail-account/update', data })

export const deleteBoxunMailAccount = (id: EntityId) =>
  request.delete<boolean>({ url: '/system/mail-account/delete?id=' + id })

export const deleteBoxunMailAccountList = (ids: EntityId[]) =>
  request.delete<boolean>({ url: '/system/mail-account/delete-list', params: { ids: ids.join(',') } })
