import {Connection, Keypair, sendAndConfirmTransaction, SystemProgram, Transaction, TransactionInstruction} from "@solana/web3.js"
import {readFileSync} from "fs"

function readKeypairFromPath(path: string): Keypair {
    const data = JSON.parse(readFileSync(path, "utf-8"))
    return Keypair.fromSecretKey(Buffer.from(data))
}

async function main() {
    const programKeypair = readKeypairFromPath(__dirname + "/../../keys/program.json")
    const aliceKeypair = readKeypairFromPath(__dirname + "/../../keys/alice.json")
    const bobKeypair = readKeypairFromPath(__dirname + "/../../keys/bob.json")
    const connection = new Connection("http://localhost:8899")
    const ix = new TransactionInstruction({
        keys: [
            {pubkey: aliceKeypair.publicKey, isSigner: true, isWritable: true},
            {pubkey: bobKeypair.publicKey, isSigner: false, isWritable: true},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ],
        programId: programKeypair.publicKey,
        data: Buffer.alloc(0),
    })
    const res = await sendAndConfirmTransaction(connection, new Transaction().add(ix), [aliceKeypair])
    console.log(res)
}

main().then(() => process.exit(0)).catch(err => console.error(err))