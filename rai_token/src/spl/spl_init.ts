import { Connection, Keypair, PublicKey, clusterApiUrl } from "@solana/web3.js";
import { mintTo, createMint, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import wallet from "../../devnet-wallet.json";

const main = async () => {
    const connection = new Connection(clusterApiUrl("devnet"), {
        commitment: "confirmed",
    });

    const payer = Keypair.fromSecretKey(new Uint8Array(wallet));

    console.log("Payer address:", payer.publicKey.toBase58());

    const mint = await createMint(
        connection,
        payer,
        payer.publicKey,
        null,
        9
    );

    console.log("Mint address:", mint.toBase58());

    const tokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        payer.publicKey
    );

    console.log("Token account address:", tokenAccount.address.toBase58());

    await mintTo(
        connection,
        payer,
        mint,
        tokenAccount.address,
        payer,
        1000000000
    );

    console.log("Minted 1 token to", tokenAccount.address.toBase58());
};

main().catch((err) => {
    console.error(err);
});
