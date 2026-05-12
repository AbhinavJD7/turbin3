import { address, createKeyPairSignerFromBytes, createSolanaRpc, createSolanaRpcSubscriptions } from "@solana/kit";
import wallet from "../../devnet-wallet.json";
import { findAssociatedTokenPda , getCreateAssociatedTokenInstructionAsync, TOKEN_PROGRAM_ADDRESS } from "@solana-program/token";

const rpc = createSolanaRpc("https://api.devnet.solana.com");

const rpcSubscriptions = createSolanaRpcSubscriptions("wss://api.devnet.solana.com");

const token_decimals = 1_000_000;
const mint = address("8Xmm4eRVSCgDSWzJZ7ADQPC1aZsLxcntcs6q8gbzwcDt");

(async () => {
        const signer = await createKeyPairSignerFromBytes(
        new Uint8Array(wallet)
    );

    const [ata] = await findAssociatedTokenPda({
        mint,
        owner: signer.address,
        tokenProgram: TOKEN_PROGRAM_ADDRESS,
    });
    console.log(`Your ATA is: ${ata}`); 

    const createAtaIx = await getCreateAssociatedTokenInstructionAsync({
        payer: signer,
        mint,
        owner: signer.address,
    });

    const { value : latestBlockhash} = await rpc.getLatestBlockhash().send();


})();