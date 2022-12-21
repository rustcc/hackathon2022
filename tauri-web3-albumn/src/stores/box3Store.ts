import { getItem, setItem } from '~/helpers/ls'

export const box3Store = defineStore('box3Store', () => {
  // const { initContract, walletAddress } = $(web3AuthStore())
  const { getJson, getStatus, storeJson } = $(useNFTStorage())
  const { addSuccess } = $(notificationStore())

  const itemsMap = $ref({})

  let currentItem = $ref({})
  let profileCid = $ref(getItem('box3.profileCid', 'ipfs://bafkreig3cwdhkurn4rjhxgoybfisttdtifxz2dw6ah5gtladfs4tcaxy34'))
  let currentCategory = $ref('')
  let needSave = $ref(false)
  let isSaving = $ref(false)
  let favoritedCidArr = $ref(getItem('box3.favoritedCidArr', [])) // TODO: 从 profile的 json 文件 读取
  let deletedCidArr = $ref(getItem('box3.deletedCidArr', [])) // TODO: 从 profile的 json 文件 读取
  let itemsCidArr = $ref(getItem('box3.itemsCidArr', []))
  const showNewItemModal = $ref(false)
  const items = $computed(() => {
    if (currentCategory === 'trash')
      return deletedCidArr.map(cid => itemsMap[cid])
    if (currentCategory === 'favorited')
      return favoritedCidArr.map(cid => itemsMap[cid])

    let arr = Object.values(itemsMap).filter(item => !deletedCidArr?.includes(item.cid))
    if (currentCategory)
      arr = arr.filter(item => item.properties.category === currentCategory)

    // todo: filter tag
    return arr
  })
  const categoriesArr = $computed(() => {
    const catArr = uniq(Object.values(itemsMap).filter(item => !item.isDeleted).map(item => item.properties.category))
    return catArr
  })
  const updateCategory = (newCategory) => {
    currentCategory = newCategory
    currentItem = items[0]
  }

  const addBoxItem = async(item) => {
    itemsCidArr.push(item.cid)
    const status = await getStatus(item.cid)
    item = {
      ...item,
      status,
    }
    setItem('box3.itemsCidArr', itemsCidArr)
    itemsMap[item.cid] = item
    currentItem = item
    needSave = true
  }

  const setCurrentItem = (item) => {
    currentItem = item
  }

  const toggleFavorited = (item) => {
    if (!item.isFavorited)
      favoritedCidArr.push(item.cid)
    else
      remove(favoritedCidArr, _cid => _cid === item.cid)

    item.isFavorited = !item.isFavorited
    setItem('box3.favoritedCidArr', favoritedCidArr)
    needSave = true
  }
  const toggleDeleted = (item) => {
    if (!item.isDeleted) deletedCidArr.push(item.cid)
    else
      deletedCidArr = filter(deletedCidArr, _cid => _cid !== item.cid)

    item.isDeleted = !item.isDeleted
    itemsMap[item.cid] = {
      ...item,
    }

    currentItem = {}
    deletedCidArr = uniq(deletedCidArr)
    setItem('box3.deletedCidArr', deletedCidArr)
    needSave = true
  }

  const saveData = async() => {
    isSaving = true
    const json = {
      itemsCidArr,
      favoritedCidArr,
      deletedCidArr,
    }
    profileCid = await storeJson(json)
    setItem('box3.profileCid', profileCid)
    await addSuccess('Save success!')
    isSaving = false
    needSave = false
  }
  onMounted(async() => {
    const _profileCid = getItem('box3.profileCid', 'ipfs://bafkreig3cwdhkurn4rjhxgoybfisttdtifxz2dw6ah5gtladfs4tcaxy34')
    if (_profileCid) {
      const rz = await getJson(_profileCid)
      itemsCidArr = rz.itemsCidArr
      favoritedCidArr = rz.favoritedCidArr
      deletedCidArr = rz.deletedCidArr
    }
  })
  watchEffect(async() => {
    if (itemsCidArr.length === 0) return
    await Promise.all(itemsCidArr.map(async(cid) => {
      const item = await getJson(cid)
      const status = await getStatus(cid)
      const isFavorited = favoritedCidArr?.includes(cid)
      const isDeleted = deletedCidArr?.includes(cid)
      itemsMap[cid] = {
        ...item,
        cid,
        status,
        isFavorited,
        isDeleted,
      }
    }))
    setCurrentItem(itemsMap[itemsCidArr[0]])
  })

  return $$({
    currentItem,
    currentCategory,
    items,
    itemsMap,
    showNewItemModal,
    favoritedCidArr,
    deletedCidArr,
    categoriesArr,
    needSave,
    isSaving,
    saveData,
    addBoxItem,
    setCurrentItem,
    toggleFavorited,
    toggleDeleted,
    updateCategory,
  })
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(box3Store, import.meta.hot))
