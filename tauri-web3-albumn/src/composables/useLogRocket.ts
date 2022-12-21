import LogRocket from 'logrocket'
import dayjs from 'dayjs'
import * as ls from '~/helpers/ls'

const getNowString = () => dayjs().format('YYYY-MM-DD HH:mm:ss')

let instance = null
export function useLogRocket () {
  if ( instance ) return instance

  const firstVisitTime = ls.getItem('logRocket-firstVisitTime', getNowString())
  
  const doInit = key => {
    if(!key) return
    // console.log('====> key :', key)
    LogRocket.init(key)
  }

  const doIdentify = address => {
    // console.log('====> address :', address)
    LogRocket.identify(address, {
      loginTime: getNowString(),
      firstVisitTime,
    })
  }
  // const doInit = () => {}
  // const doIdentify = () => {}

  instance = {
    doInit,
    doIdentify
  }
  return instance
}
