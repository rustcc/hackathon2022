import LitJsSdk from '@lit-protocol/sdk-browser'

export const litHelper = async({ chain, litNodeClient }) => {
  const doEncryptedString = async(content, accessControlConditions) => {
    const authSig = await LitJsSdk.checkAndSignAuthMessage({ chain })
    const { encryptedString, symmetricKey } = await LitJsSdk.encryptString(
      content,
    )
    const encryptedSymmetricKey = await litNodeClient.saveEncryptionKey({
      accessControlConditions,
      symmetricKey, // Uint8Array
      authSig,
      chain,
    })

    // it's a Uint8Array, turn it into a hex string now
    function blobToDataURL(blob) {
      return new Promise((resolve) => {
        const a = new FileReader()
        a.onload = function(e) { resolve(e.target.result) }
        a.readAsDataURL(blob)
      })
    }
    return {
      encryptedString: await blobToDataURL(encryptedString),
      encryptedSymmetricKey: LitJsSdk.uint8arrayToString(encryptedSymmetricKey, 'base16'),
    }
  }

  const doDecryptString = async(toDecrypt, encryptedString, accessControlConditions) => {
    const authSig = await LitJsSdk.checkAndSignAuthMessage({ chain })
    try {
      const symmetricKey = await litNodeClient.getEncryptionKey({
        accessControlConditions,
        toDecrypt,
        chain,
        authSig,
      })

      function dataURLtoBlob(dataUrl) {
        const arr = dataUrl.split(',')
        const mime = arr[0].match(/:(.*?);/)[1]
        const bstr = atob(arr[1]); let n = bstr.length; const u8arr = new Uint8Array(n)
        while (n--)
          u8arr[n] = bstr.charCodeAt(n)

        return new Blob([u8arr], { type: mime })
      }

      encryptedString = dataURLtoBlob(encryptedString)

      const decryptedString = await LitJsSdk.decryptString(
        encryptedString,
        symmetricKey,
      )
      // console.log('====> decryptedString :', decryptedString)

      return { decryptedString }
    }
    catch (err) {
      console.log('====> err :', err)
      return { err }
    }
  }

  return {
    doEncryptedString,
    doDecryptString,
  }
}
