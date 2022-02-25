import "regenerator-runtime/runtime"
import {Connection, PublicKey, SystemProgram, Transaction, TransactionInstruction,} from "@solana/web3.js"
import Wallet from "@project-serum/sol-wallet-adapter"
import lo from "buffer-layout"
import BN from "bn.js"

declare global {
  interface Window {
    solana: any
  }
}

// const connection = new Connection("http://127.0.0.1:8899")
const connection = new Connection("https://testnet.solana.com")

let solletWallet = new Wallet("https://www.sollet.io", "testnet")
solletWallet.on("connect", (publicKey) => console.log("sollet connected", publicKey.toBase58()))

export function connectPhantomWallet() {
  window.solana.on("connect", () => {
    console.log("phantom connected", window.solana.publicKey.toString())
  })
  window.solana.connect()
}

export async function connectSolletWallet() {
  await solletWallet.connect()
}

async function prepareTransaction(userPubkey: PublicKey): Promise<Transaction> {
  const bobPubkey = new PublicKey("9C8ARBpAqcmoDfqZTDtvB1JgZC7gjvcq48xRJoR7Wpeq")
  const programId = new PublicKey("Cf2FH5TEV6T511C4nJDyuyuaVc34vDA66rmmkwquyWeM")

  // encode 0.5 SOL as an input_data
  const data = Buffer.alloc(64)
  lo.ns64("value").encode(new BN("500000000"), data)

  const ix = new TransactionInstruction({
    keys: [
      { pubkey: userPubkey, isSigner: true, isWritable: true },
      { pubkey: bobPubkey, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: programId,
    data: data,
  })
  let tx = new Transaction()
  tx.add(ix)
  tx.feePayer = userPubkey
  tx.recentBlockhash = (await connection.getRecentBlockhash()).blockhash

  return tx
}

export async function sendViaPhantom() {
  console.log("sendViaPhantom called")
  const tx = await prepareTransaction(window.solana.publicKey)
  let signed = await window.solana.signTransaction(tx)
  await broadcastSignedTransaction(signed)
}

export async function sendViaSollet() {
  console.log("sendViaSollet called")
  const tx = await prepareTransaction(solletWallet.publicKey)
  let signed = await solletWallet.signTransaction(tx)
  await broadcastSignedTransaction(signed)
}

async function broadcastSignedTransaction(signed) {
  let signature = await connection.sendRawTransaction(signed.serialize())
  console.log("Submitted transaction " + signature + ", awaiting confirmation")
  await connection.confirmTransaction(signature)
  console.log("Transaction " + signature + " confirmed")
}
