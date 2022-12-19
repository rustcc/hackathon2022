export const useIdentity = (address: string, first: Number = 5, after: String = '-1') => {
  const variables = reactive({
    namespace: CYBERCONNECT_NAMESPACE,
    address,
    first,
    after,
  })
  const { data, isFetching, execute } = $(useQuery({ query: GET_IDENTITY, variables, fetchOnMount: false }))
  const followers = $computed(() => get(data, 'identity.followers.list', []))
  const followings = $computed(() => get(data, 'identity.followings.list', []))
  watchEffect(() => {
    if (!address.value) return
    execute()
  })

  return $$({
    variables,
    data,
    followers,
    followings,
    isFetching,
    execute,
  })
}
