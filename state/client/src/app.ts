import { readFileSync } from "fs"
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js"
import {
  Counter,
  decodeCounter,
  decodeSettings,
  encodeCounter,
  encodeDecIx,
  encodeIncIx,
  encodeUpdateSettingsIx,
  Settings,
} from "./serialization"
import BN from "bn.js"

function delay(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

export class App {
  static counterSeed = "counter"
  static settingsSeed = "settings"

  adminKeypair: Keypair
  userKeypair: Keypair
  programKeypair: Keypair
  connection: Connection
  counterPubkey: PublicKey
  settingsPubkey: PublicKey

  constructor() {
    this.adminKeypair = App.readKeypairFromPath(__dirname + "/../../localnet/admin.json")
    this.userKeypair = App.readKeypairFromPath(__dirname + "/../../localnet/user.json")
    this.programKeypair = App.readKeypairFromPath(__dirname + "/../../localnet/program.json")
    this.connection = new Connection("http://localhost:8899", "confirmed")
    this.counterPubkey = new PublicKey(0)
    this.settingsPubkey = new PublicKey(0)
  }

  async init() {
    this.counterPubkey = await PublicKey.createWithSeed(
      this.userKeypair.publicKey,
      App.counterSeed,
      this.programKeypair.publicKey
    )
    this.settingsPubkey = (
      await PublicKey.findProgramAddress(
        [Buffer.from(App.settingsSeed, "utf-8")],
        this.programKeypair.publicKey
      )
    )[0]
    const res = await this.connection.getAccountInfo(this.programKeypair.publicKey)
    if (!res) {
      console.error("Counter is not deployed. Deploy it first.")
      process.exit(1)
    }
    console.log("program", this.programKeypair.publicKey.toBase58())
    console.log("admin", this.adminKeypair.publicKey.toBase58())
    console.log("user", this.userKeypair.publicKey.toBase58())
    console.log("settings", this.settingsPubkey.toBase58())
    console.log("counter", this.settingsPubkey.toBase58())
  }

  async updateCounterSettings(admin: Uint8Array, inc_step: number, dec_step: number) {
    const updateSettingsIx = new TransactionInstruction({
      programId: this.programKeypair.publicKey,
      keys: [
        {
          pubkey: this.adminKeypair.publicKey,
          isSigner: true,
          isWritable: true,
        },
        { pubkey: this.settingsPubkey, isSigner: false, isWritable: true },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      data: encodeUpdateSettingsIx(admin, inc_step, dec_step),
    })

    const tx = new Transaction().add(updateSettingsIx)
    const txHash = await this.connection.sendTransaction(
      tx,
      [this.userKeypair, this.adminKeypair],
      { preflightCommitment: "max" }
    )
    console.log("update settings tx", txHash)
    await delay(3000)
  }

  async readCounterAccount(): Promise<Counter> {
    const account = await this.connection.getAccountInfo(this.counterPubkey)
    if (!account) {
      console.error("counter account is not found")
      process.exit(1)
    }
    return decodeCounter(account.data)
  }

  async readSettingsAccount(): Promise<Settings> {
    const account = await this.connection.getAccountInfo(this.settingsPubkey)
    if (!account) {
      console.error("settings account is not found")
      process.exit(1)
    }
    return decodeSettings(account.data)
  }

  async createCounterAndInc() {
    const data = encodeCounter(0, new BN(0))
    const lamports = await this.connection.getMinimumBalanceForRentExemption(data.length)
    const createAccountIx = SystemProgram.createAccountWithSeed({
      fromPubkey: this.userKeypair.publicKey,
      basePubkey: this.userKeypair.publicKey,
      seed: App.counterSeed,
      newAccountPubkey: this.counterPubkey,
      space: data.length,
      lamports: lamports,
      programId: this.programKeypair.publicKey,
    })

    const incIx = new TransactionInstruction({
      programId: this.programKeypair.publicKey,
      keys: [
        {
          pubkey: this.userKeypair.publicKey,
          isSigner: true,
          isWritable: false,
        },
        { pubkey: this.counterPubkey, isSigner: false, isWritable: true },
        { pubkey: this.settingsPubkey, isSigner: false, isWritable: false },
      ],
      data: encodeIncIx(),
    })

    const tx = new Transaction().add(createAccountIx, incIx)
    const res = await this.connection.sendTransaction(tx, [this.userKeypair])
    console.log("create counter and inc tx", res)
    await delay(3000)
  }

  async decCounter() {
    const decIx = new TransactionInstruction({
      programId: this.programKeypair.publicKey,
      keys: [
        {
          pubkey: this.userKeypair.publicKey,
          isSigner: true,
          isWritable: false,
        },
        { pubkey: this.counterPubkey, isSigner: false, isWritable: true },
        { pubkey: this.settingsPubkey, isSigner: false, isWritable: false },
      ],
      data: encodeDecIx(),
    })

    const tx = new Transaction().add(decIx)
    const res = await this.connection.sendTransaction(tx, [this.userKeypair])
    console.log("dec counter tx", res)
    await delay(3000)
  }

  static readKeypairFromPath(path: string): Keypair {
    const data = JSON.parse(readFileSync(path, "utf-8"))
    return Keypair.fromSecretKey(Buffer.from(data))
  }
}
