import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        const metadata = {
            name: "Paulo`s RUG",
            symbol: "PRUG",
            description: "Rugs rugs rugs rugs",
            image: "https://devnet.irys.xyz/Dd5Fb2eDY8B8oWDmauAos5xpE31eQLdmoEfxj5w3ZacC",
            attributes: [
                {trait_type: 'softness', value: '100'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "Dd5Fb2eDY8B8oWDmauAos5xpE31eQLdmoEfxj5w3ZacC"
                    },
                ]
            },
            creators: []
        };

        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
