export function createSessionRefresher(options: {
  apiBase: string
  getRefreshToken: () => string
  onRefreshed: (accessToken: string, refreshToken?: string) => void
  fetcher?: typeof fetch
}): () => Promise<boolean>
