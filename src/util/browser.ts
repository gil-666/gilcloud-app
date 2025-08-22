// src/utils/browser.ts
export const isClient = typeof window !== 'undefined'

// LocalStorage safe wrapper
export function getLocalStorageItem(key: string, fallback: string | null = null) {
  if (!isClient) return fallback
  try { return localStorage.getItem(key) || fallback } catch { return fallback }
}

export function setLocalStorageItem(key: string, value: string) {
  if (!isClient) return
  try { localStorage.setItem(key, value) } catch {}
}

// Window safe wrapper
export function getWindow<T>(getter: (win: Window) => T, fallback: T): T {
  return isClient ? getter(window) : fallback
}

// Example for btoa / atob
export function safeBtoa(input: string, fallback = ''): string {
  return getWindow(w => w.btoa, undefined)?.(input) ?? fallback
}

export function safeAtob(input: string, fallback = ''): string {
  return getWindow(w => w.atob, undefined)?.(input) ?? fallback
}
