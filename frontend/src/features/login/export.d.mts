import type { LoginConfiguration } from './config.mjs'

export class LoginExportError extends Error {
  readonly field: 'backgroundImage' | 'heroImage'
  constructor(field: 'backgroundImage' | 'heroImage', cause: unknown)
}

export function exportLoginConfiguration(
  value: unknown,
  options: {
    pageUrl: string
    resolveAssetUrl: (source: string) => string
    fetchAsset?: typeof fetch
    signal?: AbortSignal
  }
): Promise<LoginConfiguration>
