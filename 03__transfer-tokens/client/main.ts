import { readFileSync } from "fs"
import { ASSOCIATED_TOKEN_PROGRAM_ID, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token"
import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js"
import lo from "buffer-layout"
import BN from "bn.js"

function readKeypairFromPath(path: string): Keypair {
  const data = JSON.parse(readFileSync(path, "utf-8"))
  return Keypair.fromSecretKey(Buffer.from(data))
}

async function createAssociatedTokenAccount(
  connection: Connection,
  mint: PublicKey,
  wallet: Keypair
): Promise<PublicKey> {
  const associatedTokenAddress = await Token.getAssociatedTokenAddress(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    mint,
    wallet.publicKey
  )
  if (await connection.getAccountInfo(associatedTokenAddress)) {
    // associated token account is already created
    return associatedTokenAddress
  }

  const tokenClient = new Token(connection, mint, TOKEN_PROGRAM_ID, wallet)
  console.log("create associated token account for", wallet.publicKey.toBase58())
  return await tokenClient.createAssociatedTokenAccount(wallet.publicKey)
}

async function main() {
  const connection = new Connection("http://localhost:8899", "confirmed")
  const mint = new PublicKey("CZyEKArwVYSKkv9im3grGNXmggbPfS8YGUovBnzoKQ4s")
  const programKeypair = readKeypairFromPath(__dirname + "/../localnet/program.json")
  const aliceKeypair = readKeypairFromPath(__dirname + "/../localnet/alice.json")
  const bobKeypair = readKeypairFromPath(__dirname + "/../localnet/bob.json")
  const carolKeypair = readKeypairFromPath(__dirname + "/../localnet/carol.json")
  const aliceTokenPubkey = await createAssociatedTokenAccount(connection, mint, aliceKeypair)
  const bobTokenPubkey = await createAssociatedTokenAccount(connection, mint, bobKeypair)
  const carolTokenPubkey = await createAssociatedTokenAccount(connection, mint, carolKeypair)

  const amount = Buffer.alloc(8) // 50 SPL
  lo.ns64("value").encode(new BN("50000000000"), amount)

  // `approve` from alice to bob
  const approveIx = new TransactionInstruction({
    keys: [
      { pubkey: aliceKeypair.publicKey, isSigner: true, isWritable: true },
      { pubkey: aliceTokenPubkey, isSigner: false, isWritable: true },
      { pubkey: bobTokenPubkey, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    programId: programKeypair.publicKey,
    data: Buffer.of(1, ...amount),
  })
  const resApprove = await sendAndConfirmTransaction(connection, new Transaction().add(approveIx), [
    aliceKeypair,
  ])
  console.log("approve tx", resApprove)

  // `transfer` from alice to carol
  const transferIx = new TransactionInstruction({
    keys: [
      { pubkey: aliceKeypair.publicKey, isSigner: true, isWritable: false },
      { pubkey: aliceTokenPubkey, isSigner: false, isWritable: true },
      { pubkey: carolTokenPubkey, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    programId: programKeypair.publicKey,
    data: Buffer.of(0, ...amount),
  })
  const transferRes = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(transferIx),
    [aliceKeypair]
  )
  console.log("transfer tx", transferRes)
}

main()
  .then(() => process.exit(0))
  .catch((err) => console.error(err))
