// https://vuejs.org/guide/reusability/composables.html#conventions-and-best-practices
// https://v3.cn.vuejs.org/guide/composition-api-introduction.html#%E7%8B%AC%E7%AB%8B%E7%9A%84-computed-%E5%B1%9E%E6%80%A7
// fetch.js
import { isRef, ref, unref, watchEffect } from 'vue'

export function useFetch2(url) {
  const data = ref(null)
  const error = ref(null)

  function doFetch() {
    // reset state before fetching..
    data.value = null
    error.value = null
    // unref() unwraps potential refs
    fetch(unref(url))
      .then(res => res.json())
      .then(json => (data.value = json))
      .catch(err => (error.value = err))
  }

  if (isRef(url)) {
    // setup reactive re-fetch if input URL is a ref
    watchEffect(doFetch)
  }
  else {
    // otherwise, just fetch once
    // and avoid the overhead of a watcher
    doFetch()
  }

  return { data, error }
}

// in setup func
// import { useFeatureA } from './featureA.js'
// import { useFeatureB } from './featureB.js'
// import { useFeatureC } from './featureC.js'

// const { foo, bar } = useFeatureA()
// const { baz } = useFeatureB(foo)
// const { qux } = useFeatureC(baz)
