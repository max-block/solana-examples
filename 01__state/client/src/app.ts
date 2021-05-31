import {readFileSync} from "fs"
import {Connection, Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction} from "@solana/web3.js"
import {Counter, decodeCounter, encodeCounter, encodeIncIx, encodeSettings, encodeUpdateSettingsIx} from "./serialization";
import BN from "bn.js";

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
        this.connection = new Connection("http://localhost:8899")
        this.counterPubkey = new PublicKey(0);
        this.settingsPubkey = new PublicKey(0);
    }

    async init() {
        this.counterPubkey = await PublicKey.createWithSeed(this.adminKeypair.publicKey, App.counterSeed, this.programKeypair.publicKey)
        this.settingsPubkey = await PublicKey.createWithSeed(this.adminKeypair.publicKey, App.settingsSeed, this.programKeypair.publicKey)


        const res = await this.connection.getAccountInfo(this.programKeypair.publicKey)
        if (!res) {
            console.error("Counter is not deployed. Deploy it first.")
            process.exit(1)
        }
    }

    async createSettingsAccount() {
        if (await this.connection.getAccountInfo(this.settingsPubkey)) {
            console.log("settings account is created already", this.settingsPubkey.toBase58())
            return
        }

        const stateExample = encodeSettings(this.adminKeypair.publicKey.toBytes(), 19, 99)
        const lamports = await this.connection.getMinimumBalanceForRentExemption(stateExample.length)

        const createAccountIx = SystemProgram.createAccountWithSeed({
            fromPubkey: this.adminKeypair.publicKey,
            basePubkey: this.adminKeypair.publicKey,
            seed: App.settingsSeed,
            newAccountPubkey: this.settingsPubkey,
            space: stateExample.length,
            lamports: lamports,
            programId: this.programKeypair.publicKey,
        })

        const updateSettingsIx = new TransactionInstruction({
            programId: this.programKeypair.publicKey,
            keys: [
                {pubkey: this.adminKeypair.publicKey, isSigner: true, isWritable: true},
                {pubkey: this.settingsPubkey, isSigner: false, isWritable: true},
            ],
            data: encodeUpdateSettingsIx(19, 99),
        })

        const tx = new Transaction().add(createAccountIx, updateSettingsIx)
        const res = await this.connection.sendTransaction(tx, [this.adminKeypair])
        console.log("settings account creating", res)
    }

    async createCounterAccount() {
        if (await this.connection.getAccountInfo(this.counterPubkey)) {
            console.log("counter account is created already", this.counterPubkey.toBase58())
            return
        }

        const data = encodeCounter(this.adminKeypair.publicKey.toBytes(), new BN(0))
        const lamports = await this.connection.getMinimumBalanceForRentExemption(data.length)
        console.log({data, lamports})
        const createAccountIx = SystemProgram.createAccountWithSeed({
            fromPubkey: this.adminKeypair.publicKey,
            basePubkey: this.adminKeypair.publicKey,
            seed: App.counterSeed,
            newAccountPubkey: this.counterPubkey,
            space: data.length,
            lamports: lamports,
            programId: this.programKeypair.publicKey,
        })

        const tx = new Transaction().add(createAccountIx)
        const res = await this.connection.sendTransaction(tx, [this.adminKeypair])
        console.log("counter account creating", res)
    }


    async readCounterAccount(): Promise<Counter> {
        const account = await this.connection.getAccountInfo(this.counterPubkey)
        if (!account) {
            console.error("counter account is not found")
            process.exit(1)
        }
        console.log("account.data", Array.from(account.data))
        return decodeCounter(account.data)
    }

    async incCounter() {
        const incIx = new TransactionInstruction({
            programId: this.programKeypair.publicKey,
            keys: [
                {pubkey: this.adminKeypair.publicKey, isSigner: true, isWritable: false},
                {pubkey: this.counterPubkey, isSigner: false, isWritable: true},
                {pubkey: this.settingsPubkey, isSigner: false, isWritable: false},
            ],
            data: encodeIncIx(),
        })

        const tx = new Transaction().add(incIx)
        const res = await this.connection.sendTransaction(tx, [this.adminKeypair])
        console.log(res)
    }


    static readKeypairFromPath(path: string): Keypair {
        const data = JSON.parse(readFileSync(path, "utf-8"))
        return Keypair.fromSecretKey(Buffer.from(data))
    }
}