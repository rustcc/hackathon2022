import { ethers } from "ethers";
import MetaMaskOnboarding from "@metamask/onboarding";
import detectEthereumProvider from "@metamask/detect-provider";
import { CHAIN_ID } from "./../config/CHAIN";
import * as ls from "~/helpers/ls";

const chainId = CHAIN_ID;
const chainMap = CHAIN_MAP;

let onboarding: MetaMaskOnboarding = null;

export const useWeb3Auth = () => {
  const { addSuccess } = $(notificationStore());

  let error = $ref("");
  let isLoading = $ref(false);

  const doOnboard = async () => {
    onboarding.startOnboarding();
  };

  // new version
  let accounts = $ref([]);
  const walletAddress = $computed(() => accounts[0]);
  let userData = $ref(ls.getItem("userData", {}));
  let isShowLoginModal = $ref(false);
  let isShowChainSwitchModal = $ref(false);
  let isShowOnboardModal = $ref(false);
  let rawProvider = $ref(null);
  let web3Provider = $ref(null);
  let signer = $ref(null);

  const doSwitchChain = async () => {
    if (isLoading) return;
    isLoading = true;
    error = "";

    const rawProvider = await detectEthereumProvider();
    try {
      await rawProvider.request({
        method: "wallet_switchEthereumChain",
        params: [{ chainId }],
      });
      isShowChainSwitchModal = false;
      isLoading = false;
      return true;
    } catch (switchError) {
      // This error code indicates that the chain has not been added to MetaMask.
      if (switchError.code === 4902) {
        try {
          await rawProvider.request({
            method: "wallet_addEthereumChain",
            params: [chainMap[chainId]],
          });
          return true;
        } catch (addError) {
          error = addError.message;
          return false;
        }
      }
      error = switchError.message;
    }

    isLoading = false;
    return false;
  };

  const handleAccountsChanged = (newAccounts) => {
    const oldAccounts = ls.getItem("accounts", []);
    console.log('====> oldAccounts, newAccounts :', oldAccounts, newAccounts)
    // means user do not login yet, should notify user about login success
    if (oldAccounts.length === 0) addSuccess("Login Success!");

    accounts = newAccounts;
    ls.setItem("accounts", newAccounts);

    // means user click on logout btn
    if (newAccounts.length === 0) {
      addSuccess("Logout Success!");
      isShowLoginModal = true;
    } else {
      isShowLoginModal = false;
    }
  };

  const doRequestAccounts = async () => {
    const provider = await detectEthereumProvider();
    try {
      const rz = await provider.request({ method: "eth_requestAccounts" });
      handleAccountsChanged(rz);
    } catch (err) {
      error = err.message;
      return false;
    }

    web3Provider = new ethers.providers.Web3Provider(provider);
    signer = web3Provider.getSigner();

    return provider;
  };

  const doLogin = async () => {
    if (isLoading) return;
    isLoading = true;
    await doRequestAccounts();
    isLoading = false;
    return true;
  };

  const doLogout = async () => {
    handleAccountsChanged([]);
  };

  const initContract = async (key, isWrite = false) => {
    const contractAddress = get(CHAIN_CONTRACT_MAP, `${key}.${CHAIN_ID}`);
    const contractAbi = CHAIN_CONTRACT_ABI_MAP[key];
    if (!contractAbi) throw new Error(`abi not found for ${key}`);

    let provider = "";
    try {
      const rawProvider = await detectEthereumProvider();
      provider = new ethers.providers.Web3Provider(rawProvider);
    } catch (err) {
      isShowOnboardModal = true; // TODO: fix detectEthereumProvider bug
      return false;
    }

    if (!isWrite)
      return new ethers.Contract(contractAddress, contractAbi, provider);

    if (!walletAddress) {
      await initWeb3();
      return;
      // const rz = await doLogin();
      // if (!rz) return;
    }

    if (!walletAddress) throw new Error("user do not login yet");

    const signer = provider.getSigner();

    return new ethers.Contract(contractAddress, contractAbi, signer);
  };

  const initWeb3 = async () => {
    onboarding = new MetaMaskOnboarding();
    if (!MetaMaskOnboarding.isMetaMaskInstalled()) {
      isShowOnboardModal = true;
      return;
    }

    let provider = "";
    try {
      provider = await detectEthereumProvider();
      if (!provider) return false;

      web3Provider = new ethers.providers.Web3Provider(provider);
      signer = web3Provider.getSigner();
      provider.on("message", (msg) => {
        console.log("====> msg :", msg);
      });
      provider.on("accountsChanged", handleAccountsChanged);
      provider.on("chainChanged", () => {
        isLoading = false;
        if (provider.chainId !== chainId) {
          isShowLoginModal = false;
          isShowChainSwitchModal = true;
        } else {
          isShowChainSwitchModal = false;
          isShowLoginModal = true;
        }
      });
    } catch (err) {
      console.log("====> err :", err);
      return false;
    }

    if (provider.chainId !== chainId) {
      isShowChainSwitchModal = true;
      return;
    }

    accounts = ls.getItem("accounts", []);
    if (accounts.length === 0) {
      isShowLoginModal = true;
      return;
    }
  };
  const removeWeb3EventListener = async () => {
    const provider = await detectEthereumProvider();
    if (!provider) return;

    if (provider.off) {
      provider.off("accountsChanged", handleAccountsChanged);
    }
  };

  const getTxUrl = (hash) => {
    const prefixUrl = chainMap[chainId]["blockExplorerUrls"];
    return `${prefixUrl}tx/${hash}`;
  };

  watchEffect(() => {
    if (!MetaMaskOnboarding.isMetaMaskInstalled()) return;
    if (accounts.length > 0) onboarding.stopOnboarding();
  });
  
  return $$({
    initWeb3,
    removeWeb3EventListener,
    doSwitchChain,
    initContract,
    getTxUrl,
    error,
    rawProvider,
    userData,
    isLoading,
    doOnboard,
    chainId,
    doLogin,
    doLogout,
    isShowOnboardModal,
    isShowChainSwitchModal,
    isShowLoginModal,
    web3Provider,
    signer,
    walletAddress,
    ethers,
  });
};
