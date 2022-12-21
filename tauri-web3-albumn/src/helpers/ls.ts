export const setItem = (key: string | any, val = '') => {
  if (typeof window === 'undefined')
    return false

  if (typeof key === 'object') {
    for (const _key in key)
      setItem(_key, key[_key])

    return true
  }

  val = JSON.stringify(val)
  window.localStorage.setItem(key, val)
}

export const getItem = (key: string, defaultVal = '') => {
  if (typeof window === 'undefined')
    return defaultVal

  let val = window.localStorage.getItem(key)
  if (val === 'undefined' || val === null)
    return defaultVal

  val = JSON.parse(val)
  // console.log(`getItem ====> ${key}:`, val)

  return val
}

// ls.setItem('aString', 'a string')
// ls.setItem('aInt', 10)
// ls.setItem('aFloat', 10.00001)
// ls.setItem('anArray', ['aaa', 'bbbb'])
// ls.setItem('anObject', { aaa: 'valA', bbbb: 'valB' })
// const key = 'key value'
// setItem('aaaa', 'bbb')
// setItem({ key })

// const rz1 = ls.getItem('aString', 'a string')
// const rz2 = ls.getItem('aInt', 10)
// const rz3 = ls.getItem('aFloat', 10.00001)
// const rz4 = ls.getItem('anArray', ['aaa', 'bbbb'])
// const rz5 = ls.getItem('anObject', { aaa: 'valA', bbbb: 'valB' })
