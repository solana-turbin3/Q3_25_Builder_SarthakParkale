import { Transaction, SystemProgram, Connection, Keypair,
LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from
"@solana/web3.js"
import wallet from "./dev-wallet.json";
// import wallet from "./Turbin3-wallet.json"
const from = Keypair.fromSecretKey(new Uint8Array(wallet));
const to = new
PublicKey("3GkUrSgQpdSFVg7xmc5Zfe6SDQV61GE757VKCMfZ4K48");
const connection = new Connection("https://api.devnet.solana.com");
(async () => {
try {
const transaction = new Transaction().add(
SystemProgram.transfer({
fromPubkey: from.publicKey,

toPubkey: to,
lamports: LAMPORTS_PER_SOL / 100,
})
);
transaction.recentBlockhash = (
await connection.getLatestBlockhash('confirmed')
).blockhash;
transaction.feePayer = from.publicKey;
const signature = await sendAndConfirmTransaction(
connection,
transaction,
[from]
);
console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${signature}?cluster=devnet`);
} catch (e) {
console.error(`Oops, something went wrong: ${e}`);
}
})();