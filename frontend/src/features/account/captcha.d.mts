export type CaptchaType = 'blockPuzzle' | 'clickWord'
export interface CaptchaConfig {
  enabled: boolean
  type: CaptchaType
}
export interface CaptchaPoint {
  x: number
  y: number
}
export interface CaptchaChallenge {
  type: CaptchaType
  token: string
  secretKey?: string
  originalImageBase64: string
  jigsawImageBase64?: string
  wordList?: string[]
  expiresAt: number
}
export const CAPTCHA_WIDTH: number
export const CAPTCHA_HEIGHT: number
export const CAPTCHA_LIFETIME: number
export function parseCaptchaConfig(value: unknown): CaptchaConfig
export function scaleCaptchaPoint(x: number, y: number, width: number, height: number): CaptchaPoint
export function createCaptchaSession(options: {
  type: CaptchaType
  post: (url: string, data: object, signal: AbortSignal) => Promise<unknown>
  now?: () => number
  clientUid?: string
}): {
  load(): Promise<CaptchaChallenge>
  verify(points: CaptchaPoint | CaptchaPoint[]): Promise<string>
  cancel(): void
}
