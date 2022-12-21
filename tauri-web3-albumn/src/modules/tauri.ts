
export const install: UserModule = ({ isClient, app, router }) => {
  if (!isClient) return
  if (!window.__TAURI__) return
  const { invoke } = window.__TAURI__.tauri
  invoke('greet', { name: 'World' })
}
