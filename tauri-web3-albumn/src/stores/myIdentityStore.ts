export const myIdentityStore = defineStore('myIdentityStore', () => {
  const { walletAddress } = $(web3Store())

  const { data, execute, isFetching } = $(useIdentity($$(walletAddress)))
  const identity = $computed(() => data && data.identity)

  return $$({
    identity,
    execute,
    isFetching,
  })
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(myIdentityStore, import.meta.hot))
