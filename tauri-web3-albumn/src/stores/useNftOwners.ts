export const useNftOwners = defineStore('nftOwners', () => {
  const variables = reactive({
    contract: '',
    tokenId: '',
  })
  const { data, isFetching, execute } = useQuery({ query: GET_NFT_OWNERS, variables })

  return {
    variables,
    data,
    isFetching,
    execute,
  }
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(useNftOwners, import.meta.hot))
