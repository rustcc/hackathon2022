// import keccak256 from 'keccak256' // Keccak256 hashing
// import MerkleTree from 'merkletreejs' // MerkleTree.js
import { ethers } from 'ethers'

export function init(whiteList) {
  let data = ''
  let merkleTree = ''
  const generateLeaf = (address: string, value: string): Buffer => {
    return Buffer.from(
    // Hash in appropriate Merkle format
      ethers.utils
        .solidityKeccak256(['address', 'uint256'], [address, value])
        .slice(2),
      'hex',
    )
  }

  const generateAmountAndProof = (address) => {
    address = ethers.utils.getAddress(unref(address).toString())
    const mintAmount = whiteList[address]

    if (!mintAmount) {
      return {
        mintAmount,
      }
    }
    // Generate hashed leaf from address
    const leaf: Buffer = generateLeaf(address, mintAmount)
    // Generate airdrop proof
    const proof: string[] = merkleTree.getHexProof(leaf)
    return { mintAmount, proof }
  }

  data = Object.entries(whiteList).map(([address, amount]) =>
    generateLeaf(
      ethers.utils.getAddress(address),
      amount,
    ),
  )
  merkleTree = new MerkleTree(data, keccak256, { sortPairs: true })

  return {
    merkleTree,
    generateLeaf,
    generateAmountAndProof,
  }
}
