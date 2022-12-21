import { acceptHMRUpdate, defineStore } from "pinia";
import * as ls from "~/helpers/ls";

const storeName = "user";
const cacheData = ls.getItem(storeName, {
  name: "",
  avatar: "",
  description: "",
});

export const userStore = defineStore(storeName, () => {
  const { walletAddress } = $(web3AuthStore());

  let name = $ref(cacheData.name);
  let avatar = $ref(cacheData.avatar);
  let description = $ref(cacheData.description);
  const updateCache = (data) => {
    Object.keys(data).forEach((key) => {
      cacheData[key] = data[key];
    });
    ls.setItem(storeName, cacheData);
  };
  return $$({
    walletAddress,
    name,
    avatar,
    description,
    updateCache,
  });
});

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(userStore, import.meta.hot));
