export function safeRedirect(value: unknown, fallback?: string): string
export function sameSessionIdentity(previous: object, next: object): boolean
export function sessionIsActive(
  mode: string,
  session: {
    signedOut?: boolean
    mode?: string
    accessToken?: string
  }
): boolean
