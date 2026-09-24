export type RecordStatus = 'enabled' | 'disabled'
export interface ExampleCategory {
  id: string
  parentId: string | null
  name: string
}
export interface RecordInput {
  name: string
  code: string
  categoryId: string
  status: RecordStatus
}
export interface ExampleRecord extends RecordInput {
  id: string
  updatedAt: string
}
export interface ExampleState {
  version: number
  categories: ExampleCategory[]
  records: ExampleRecord[]
}
export interface CategoryNode extends ExampleCategory {
  children: CategoryNode[]
  recordCount: number
}
export interface RecordQuery {
  keyword?: string
  categoryId?: string | null
  status?: RecordStatus | ''
  page?: number
  pageSize?: number
}
export interface RecordPage {
  items: ExampleRecord[]
  total: number
  page: number
  pageSize: number
}
export interface ExampleStorage {
  getItem(key: string): string | null
  setItem(key: string, value: string): void
}
export class ExampleDataError extends Error {
  key: string
  constructor(key: string)
}
export function createSeedState(): ExampleState
export function categoryIdsWithDescendants(
  categories: ExampleCategory[],
  categoryId: string
): Set<string>
export function categoryOptions(categories: ExampleCategory[]): { value: string; label: string }[]
export function buildCategoryTree(state: ExampleState): CategoryNode[]
export function queryRecords(state: ExampleState, query?: RecordQuery): RecordPage
export function validateRecordInput(
  state: ExampleState,
  input: RecordInput,
  recordId?: string | null
): void
export function saveRecord(
  state: ExampleState,
  input: RecordInput,
  recordId?: string | null,
  now?: Date
): ExampleState
export function deleteRecord(state: ExampleState, recordId: string): ExampleState
export function saveCategory(
  state: ExampleState,
  input: { name: string; parentId: string | null },
  categoryId?: string | null
): ExampleState
export function deleteCategory(state: ExampleState, categoryId: string): ExampleState
export function readState(storage: ExampleStorage, key: string): ExampleState
export function writeState(storage: ExampleStorage, key: string, state: ExampleState): void
