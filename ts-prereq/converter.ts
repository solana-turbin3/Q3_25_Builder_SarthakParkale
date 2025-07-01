import bs58 from "bs58";
import * as fs from "fs";
import prompt from "prompt-sync";

const input = prompt()("Paste your Phantom private key: ");
const decoded = bs58.decode(input);
console.log("Uint8Array:", Array.from(decoded));

fs.writeFileSync("Turbin3-wallet.json", JSON.stringify(Array.from(decoded)));
console.log("✅ Saved to Turbin3-wallet.json");
