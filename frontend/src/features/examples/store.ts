import { ref } from 'vue'
import { appConfig } from '@/config'
import {
  createSeedState,
  deleteCategory,
  deleteRecord,
  ExampleDataError,
  readState,
  saveCategory,
  saveRecord,
  writeState,
  type ExampleState,
  type RecordInput
} from './model.mjs'

const storageKey = `${appConfig.appId}:examples:v1`
const state = ref<ExampleState>(createSeedState())
const storageError = ref<string | null>(null)

try {
  state.value = readState(window.localStorage, storageKey)
} catch (error) {
  storageError.value = error instanceof ExampleDataError ? error.key : 'examples.errors.storageRead'
}

function commit(next: ExampleState) {
  try {
    writeState(window.localStorage, storageKey, next)
  } catch {
    storageError.value = 'examples.errors.storageWrite'
    throw new ExampleDataError('examples.errors.storageWrite')
  }
  state.value = next
  storageError.value = null
}

export function errorKey(error: unknown): string {
  return error instanceof ExampleDataError ? error.key : 'examples.errors.operationFailed'
}

export function useExamples() {
  return {
    state,
    storageError,
    saveRecord(input: RecordInput, id: string | null = null) {
      const next = saveRecord(state.value, input, id)
      commit(next)
    },
    deleteRecord(id: string) {
      const next = deleteRecord(state.value, id)
      commit(next)
    },
    saveCategory(input: { name: string; parentId: string | null }, id: string | null = null) {
      const next = saveCategory(state.value, input, id)
      commit(next)
    },
    deleteCategory(id: string) {
      const next = deleteCategory(state.value, id)
      commit(next)
    }
  }
}
