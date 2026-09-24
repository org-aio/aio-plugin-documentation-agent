export interface AioRoutePage {
  id: string
  route: string
}

export function createAioEntryResolver(
  pages: readonly AioRoutePage[],
  prefix?: string
): (pathname: string, search?: string) => string | null
