import MetaMaskOnboarding from '@metamask/onboarding'
import type { UserModule } from '~/types'

const chains = [
  {
    id: '0x1',
    token: 'ETH',
    label: 'Ethereum Mainnet',
    rpcUrl: 'https://mainnet.infura.io/v3/29cf28093f924cb78e1b0679dc433bc1',
  },
  {
    id: '0x4',
    token: 'rETH',
    label: 'Ethereum Rinkeby Testnet',
    rpcUrl: 'https://rinkeby.infura.io/v3/29cf28093f924cb78e1b0679dc433bc1',
  },
  {
    id: '0x38',
    token: 'BNB',
    label: 'BNB Chain',
    rpcUrl: 'https://bsc-dataseed.binance.org/',
  },
  {
    id: '0x89',
    token: 'MATIC',
    label: 'Matic Mainnet',
    rpcUrl: 'https://matic-mainnet.chainstacklabs.com',
  },
  {
    id: '0xfa',
    token: 'FTM',
    label: 'Fantom Mainnet',
    rpcUrl: 'https://rpc.ftm.tools/',
  },
]

export const install: UserModule = ({ isClient, app }) => {
  if (!isClient) {
    app.provide('vw3', {})
    return
  }

  const onboarding = new MetaMaskOnboarding()
  onboarding.isMetaMaskInstalled = MetaMaskOnboarding.isMetaMaskInstalled
  app.provide('onboarding', onboarding)
}
