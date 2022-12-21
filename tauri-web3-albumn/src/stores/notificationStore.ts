import { findIndex } from 'lodash'

interface MessageItem { title: string; timeout?: number; type?: string; _id?: number }

const timerMap = {}
export const notificationStore = defineStore('notificationStore', () => {
  const items = $ref<[MessageItem]>([])
  const addItem = (item: item) => {
    if (!item._id)
      item._id = parseInt(`${Date.now()}${Math.floor(Math.random() * 10000)}`, 10)

    const timeout = (item.timeout || 3) + 1

    items.push(item)
    timerMap[item._id] = setTimeout(() => {
      removeItem(item)
    }, timeout * 1000)
  }

  const removeItem = (item: MessageItem) => {
    const index = findIndex(items, _item => item._id === _item._id)
    if (index === -1) return
    items.splice(index, 1)
    if (timerMap[item._id]) {
      clearTimeout(timerMap[item._id])
      timerMap[item._id] = null
      delete timerMap[item._id]
    }
  }

  const addSuccess = title => {
    addItem({
      title,
      type: 'success'
    })
  }
  // const itemsMock = [
  //   { title: 'msg1', timeout: 3, type: 'error' },
  //   { title: 'msg2', timeout: 4, type: 'success' },
  //   { title: 'msg3', timeout: 5, type: 'warning' },
  // ]
  // itemsMock.forEach(item => addItem(item))

  return $$({
    addSuccess,
    addItem,
    removeItem,
    items,
  })
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(notificationStore, import.meta.hot))
