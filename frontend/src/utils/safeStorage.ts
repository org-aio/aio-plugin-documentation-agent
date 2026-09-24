export interface StorageLike {
  getItem(key: string): string | null
  setItem(key: string, value: string): void
  removeItem(key: string): void
}

export function createSafeStorage(kind: 'local' | 'session'): StorageLike {
  const fallback = new Map<string, string>()
  const memory: StorageLike = {
    getItem: (key) => fallback.get(key) ?? null,
    setItem: (key, value) => {
      fallback.set(key, value)
    },
    removeItem: (key) => {
      fallback.delete(key)
    }
  }
  try {
    const storage = kind === 'local' ? globalThis.localStorage : globalThis.sessionStorage
    const probe = `__aio_storage_probe__:${kind}`
    storage.setItem(probe, '1')
    storage.removeItem(probe)
    return storage
  } catch {
    return memory
  }
}

export const safeLocalStorage = createSafeStorage('local')
export const safeSessionStorage = createSafeStorage('session')
