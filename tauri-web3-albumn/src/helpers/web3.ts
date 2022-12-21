import { ethers } from 'ethers'

// parseEther('0.01')
export const parseEther = val => ethers.utils.parseEther(val)
export const formatEther = val => ethers.utils.formatEther(val)

export const formatUnits = function(val, precision = 18, fraction = 4) {
  if (!val) return ''
  val = ethers.utils.formatUnits(val, precision)
  val = val.split('.')
  val = `${val[0]}.${val[1].substr(0, fraction)}`
  return val
}

// 37462 => 37.5K
export const numberFormat = (num) => {
  return new Intl.NumberFormat('en-US', { maximumFractionDigits: 1, notation: 'compact', compactDisplay: 'short' }).format(num)
}

export const parseUnits = function(val, precision = 18) {
  if (!val)
    val = 0

  return ethers.utils.parseUnits(val.toString(), precision)
}

export const shortAddress = address => address ? `${address.substr(0, 6)}...${address.substr(-4)}` : ''

// export const chatLink = address => `https://chat.web3nft.social/dm/${address}`
export const chatLink = (address) => {
  if (!address) return ''
  address = ethers.utils.getAddress(address)
  return `https://chat.web3nft.social/dm/${address}`
  // return `https://chat.blockscan.com/index?a=${address}`
}

export const isSameAddress = (a, b) => {
  a = unref(a)
  b = unref(b)
  if (!a || !b)
    return false

  a = ethers.utils.getAddress(a)
  b = ethers.utils.getAddress(b)
  return a === b
}

export const isValidateAddress = (address) => {
  try {
    return ethers.utils.getAddress(unref(address))
  }
  catch (e) {
    return false
  }
}
