import wallet from "C:/Users/sarth/Solana-Starter/solana-starter/ts/cluster1/wallet/turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        const image= await readFile()
        //2. Convert image to generic file.
        const genericFile=createGenericFile(image,"C:/Users/sarth/Solana-Starter/solana-starter/generug.png",{
            contentType:"image/png"
        });
        //3. Upload image
        const [myUri] = await umi.uploader.upload([genericFile]) 
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
