import { computed, ref } from 'vue'
import request from '@/config/axios'

export type ProjectRecord = {
  id: string | number
  projectName?: string
  name?: string
  [key: string]: unknown
}

export type ProjectCompanyRecord = {
  id: string | number
  projectId?: string | number
  unitName?: string
  unitType?: number
  address?: string
  contactPerson?: string
  phone?: string
  [key: string]: unknown
}

export const PROJECT_STORAGE_KEY = 'boxun:current-project'

// 旧系统通过 sessionStorage.projectID 在多个页面间共享当前项目；
// 这里沿用该语义，但用带应用前缀的 key，避免和其它系统冲突。
const readStoredProjectId = (): string => {
  try {
    return sessionStorage.getItem(PROJECT_STORAGE_KEY) || ''
  } catch {
    return ''
  }
}

export const currentProjectId = ref(readStoredProjectId())
export const projects = ref<ProjectRecord[]>([])
export const projectsLoading = ref(false)
export const projectsLoaded = ref(false)

export const projectContext = {
  currentProjectId,
  projects,
  projectsLoading,
  projectsLoaded,
  currentProject: computed(
    () => projects.value.find((item) => String(item.id) === currentProjectId.value) ?? null
  )
}

export const loadProjects = async (force = false): Promise<ProjectRecord[]> => {
  if (projectsLoading.value) {
    return projects.value
  }
  if (projectsLoaded.value && !force) {
    return projects.value
  }
  projectsLoading.value = true
  try {
    const list = await request.get<ProjectRecord[]>({ url: '/boxun/project-info/get-all-project' })
    projects.value = Array.isArray(list) ? list : []
    projectsLoaded.value = true
    const exists = projects.value.some((item) => String(item.id) === currentProjectId.value)
    if (!exists) {
      setCurrentProject(projects.value[0] ? String(projects.value[0].id) : '')
    }
    return projects.value
  } finally {
    projectsLoading.value = false
  }
}

export const setCurrentProject = (projectId: string | number | undefined | null): void => {
  const value = projectId == null ? '' : String(projectId)
  currentProjectId.value = value
  try {
    if (value) {
      sessionStorage.setItem(PROJECT_STORAGE_KEY, value)
    } else {
      sessionStorage.removeItem(PROJECT_STORAGE_KEY)
    }
  } catch (error) {
    console.warn('无法保存当前项目，本次切换仅在当前页面生效。', error)
  }
  window.dispatchEvent(new CustomEvent('boxun:project-changed', { detail: { projectId: value } }))
}

export const loadProjectCompanies = async (
  projectId?: string | number | null
): Promise<ProjectCompanyRecord[]> => {
  const value = projectId == null ? currentProjectId.value : String(projectId)
  if (!value) {
    return []
  }
  return request.get<ProjectCompanyRecord[]>({
    url: '/boxun/project-company/simple-list',
    params: { projectId: value }
  })
}

export const projectUnitType = {
  construction: 1,
  development: 2,
  witness: 3,
  inspection: 4
} as const
