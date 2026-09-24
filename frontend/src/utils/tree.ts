export const defaultProps = {
  children: 'children',
  label: 'name',
  value: 'id'
}

export const handleTree = <T extends object>(
  data: T[],
  id = 'id',
  parentId = 'parentId',
  children = 'children',
  rootId: unknown = 0
): T[] => {
  const nodes = data.map((item) => ({ ...item })) as Record<string, unknown>[]
  const byId = new Map(nodes.map((item) => [item[id], item]))
  const roots: Record<string, unknown>[] = []

  for (const node of nodes) {
    const parentKey = node[parentId]
    const parent = byId.get(parentKey)
    if (!parent || parentKey === rootId || parentKey === null || parentKey === undefined) {
      roots.push(node)
      continue
    }
    const currentChildren = (parent[children] as Record<string, unknown>[] | undefined) ?? []
    currentChildren.push(node)
    parent[children] = currentChildren
  }
  return roots as T[]
}
