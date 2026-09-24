export interface AioRoutePage {
  id: string
  route: string
}

export function createAioEntryResolver(
  pages: readonly AioRoutePage[],
  prefix?: string
): (pathname: string, search?: string) => string | null

export function resolveAioDocumentRoute(
  documentValue: Pick<Document, 'querySelector'>,
  pages: readonly AioRoutePage[]
): string | null
