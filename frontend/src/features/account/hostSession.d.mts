export interface HostSessionBridge {
  request(payload: {
    method: string
    path: string
    query?: string | null
    body?: Uint8Array
  }): Promise<{ status: number; body: Uint8Array }>
}

export interface HostSessionToken {
  accessToken: string
  refreshToken?: string
  userId?: number | string
}

export function requestHostSession(
  bridge: HostSessionBridge | null | undefined,
  apiBase?: string
): Promise<HostSessionToken | null>

export function createHostSessionLoader(
  bridge: HostSessionBridge | null | undefined,
  apiBase?: string
): () => Promise<HostSessionToken | null>
