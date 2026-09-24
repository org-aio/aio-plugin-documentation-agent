import { getBoxunProjectInfoSimpleList } from '@/pages/boxun/project-info/BoxunProjectInfoApi'

export const TEMPLATE_TYPE_NAMES: Record<number, string> = {
  1: '委托单',
  2: '见证记录',
  3: '旁站记录',
  4: '混凝土施工记录'
}

export const templateTypeName = (value?: number | string | null) => {
  const code = Number(value)
  return TEMPLATE_TYPE_NAMES[code] ?? value ?? '-'
}

export const loadProjectOptions = async () => {
  const projects = await getBoxunProjectInfoSimpleList()
  return projects.map((project) => ({
    id: String(project.id),
    name: String(project.projectName ?? project.id)
  }))
}

export const loadProjectNames = async () =>
  Object.fromEntries((await loadProjectOptions()).map((project) => [project.id, project.name]))
