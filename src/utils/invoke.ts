export async function invoke<T>(cmd: string, args: any = {}): Promise<T> {
  const isTauri = typeof window !== 'undefined' && (window as any).__TAURI__;
  if (isTauri) {
    const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
    return tauriInvoke(cmd, args);
  } else {
    // Determine the host to send requests to.
    // In production web mode, it is the same host and port.
    const url = `/api/invoke`;
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer ' + (JSON.parse(localStorage.getItem('app_settings') || '{}').adminToken || ''),
      },
      body: JSON.stringify({ cmd, args }),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const result = await response.json();
    if (result.success !== undefined) {
      if (!result.success) {
        throw new Error(result.error || 'Unknown error from invoke API');
      }
      return result.data as T;
    }

    // In case the API returns the result directly
    return result as T;
  }
}
export function convertFileSrc(path: string): string {
    return `/api/convertFileSrc?path=${encodeURIComponent(path)}`;
}
