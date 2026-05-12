import { createSignerFromKeypair, publicKey, signerIdentity } from "@metaplex-foundation/umi";
import wallet from "../../devnet-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { CreateMetadataAccountV3InstructionAccounts, CreateMetadataAccountV3InstructionArgs, DataV2Args, createMetadataAccountV3 } from "@metaplex-foundation/mpl-token-metadata";
import bs58 from 'bs58';
 const mint = publicKey("8Xmm4eRVSCgDSWzJZ7ADQPC1aZsLxcntcs6q8gbzwcDt");
 const umi = createUmi("https://api.devnet.solana.com");

 const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet)) //get keypair and convert it into umi compatible format
 // convert keypair into signer
 const signer = createSignerFromKeypair(umi,keypair);
 umi.use(signerIdentity(signer));

 (async () => {
    try{
        const accounts : CreateMetadataAccountV3InstructionAccounts = {
            mint,
            mintAuthority: signer
        }

        const data : DataV2Args = {
            name: "Rai coin",
            symbol: "CAT",
            uri : "",
            sellerFeeBasisPoints: 1,
            creators: null,
            collection: null,
            uses: null
        }

        const args :CreateMetadataAccountV3InstructionArgs = { 
            data,
            isMutable: true,
            collectionDetails: null
        };

        const tx = createMetadataAccountV3(umi, {
            ...accounts,
            ...args
        });

        const result = await tx.sendAndConfirm(umi);
        console.log("result:", result);
        console.log("Simplified result:" , bs58.encode(Buffer.from(result.signature)));
        
    }
    catch(error){
        console.log("error:",error);
    }
 })();

 //45j69gbMbzviQhJJHWATj1b3YNYrF54ZaFpVNearQGSLmVAsnidgz9K3fr7dDj8LiUCCWMCq8yJUbmYDvbXu3V8m
 