import { appConfig } from '@/config'
import { getSessionState } from './auth'

const matches = (granted: string[], expected: string[]): boolean => {
  return granted.includes('*:*:*') || expected.some((permission) => granted.includes(permission))
}

export const checkPermi = (permissions: string[]): boolean => {
  if (appConfig.dataMode === 'demo') {
    return true
  }
  const granted = getSessionState().permissions
  return Array.isArray(granted) && matches(granted, permissions)
}

export const checkRole = (roles: string[]): boolean => {
  if (appConfig.dataMode === 'demo') {
    return true
  }
  const granted = getSessionState().roles
  return (
    Array.isArray(granted) &&
    (granted.includes('super_admin') || roles.some((role) => granted.includes(role)))
  )
}
