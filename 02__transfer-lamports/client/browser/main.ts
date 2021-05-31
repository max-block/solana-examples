import 'regenerator-runtime/runtime'
import {Connection, PublicKey, SystemProgram, Transaction, TransactionInstruction} from '@solana/web3.js'

declare global {
    interface Window {
        solana: any;
    }
}

export function connectWallet() {
    window.solana.on("connect", () => {
        console.log("phantom connected!")
        console.log(window.solana.publicKey.toString())
    })
    window.solana.connect()
}

export async function sendTx() {
    console.log("sendTx called")
    const provider = window.solana;
    const bobPubkey = new PublicKey("9C8ARBpAqcmoDfqZTDtvB1JgZC7gjvcq48xRJoR7Wpeq");
    const programId = new PublicKey("Cf2FH5TEV6T511C4nJDyuyuaVc34vDA66rmmkwquyWeM");
    const connection = new Connection("http://localhost:8899")
    const ix = new TransactionInstruction({
        keys: [
            {pubkey: provider.publicKey, isSigner: true, isWritable: true},
            {pubkey: bobPubkey, isSigner: false, isWritable: true},
            {pubkey: SystemProgram.programId, isSigner:false, isWritable: false},
        ],
        programId: programId,
        data: Buffer.alloc(0),
    })
    let transaction = new Transaction()
    transaction.add(ix);
    transaction.feePayer = provider.publicKey;
    transaction.recentBlockhash = (await connection.getRecentBlockhash()).blockhash;
    console.log(transaction)
    console.log("Sending signature request to wallet");
    let signed = await provider.signTransaction(transaction);
    console.log("Got signature, submitting transaction");
    let signature = await connection.sendRawTransaction(signed.serialize());
    console.log(
        "Submitted transaction " + signature + ", awaiting confirmation"
    );
    await connection.confirmTransaction(signature);
    console.log("Transaction " + signature + " confirmed");
}