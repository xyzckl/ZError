export async function invoke<T>(cmd: string, args: Record<string, any> = {}): Promise<T> {
  const hasTauri = typeof window !== 'undefined' && (window as any).__TAURI_INTERNALS__;

  if (hasTauri) {
    const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
    return tauriInvoke<T>(cmd, args);
  } else {
    const token = localStorage.getItem('token');
    const response = await fetch('/api/invoke', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(token ? { 'Authorization': `Bearer ${token}` } : {})
      },
      body: JSON.stringify({ cmd, args })
    });

    if (!response.ok) throw new Error(`HTTP Error: ${response.status}`);
    const result = await response.json();
    if (result.success === false) throw new Error(result.error || 'Unknown error');
    return result.data as T;
  }
}
