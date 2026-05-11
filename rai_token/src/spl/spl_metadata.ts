import { Connection, Keypair, clusterApiUrl } from "@solana/web3.js";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { mplTokenMetadata, createV1, CreateV1InstructionAccounts, CreateV1InstructionArgs, TokenStandard } from "@metaplex-foundation/mpl-token-metadata";
import { keypairIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi";
import wallet from "../../devnet-wallet.json";

const main = async () => {
    const connection = new Connection(clusterApiUrl("devnet"), {
        commitment: "confirmed",
    });

    const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

    console.log("Payer address:", keypair.publicKey.toBase58());

    const umi = createUmi(connection.rpcEndpoint)
        .use(mplTokenMetadata());

    const umiKeypair = umi.eddsa.createKeypairFromSecretKey(keypair.secretKey);
    umi.use(keypairIdentity(umiKeypair));

    const mint = generateSigner(umi);
    console.log("Mint address:", mint.publicKey);

    const createV1InstructionArgs: CreateV1InstructionArgs = {
        name: "RAI Token",
        symbol: "RAI",
        uri: "https://raw.githubusercontent.com/Abhinav-rai-03/token-metadata/main/metadata.json",
        sellerFeeBasisPoints: percentAmount(0),
        tokenStandard: TokenStandard.Fungible,
        isMutable: true,
        creators: null,
        ruleSet: null,
        uses: null,
    };

    const createV1InstructionAccounts: CreateV1InstructionAccounts = {
        mint: mint,
    };

    const createTx = createV1(umi, {
        ...createV1InstructionAccounts,
        ...createV1InstructionArgs,
    });

    const tx = await createTx.sendAndConfirm(umi);

    console.log("Transaction signature:", tx.signature);
};

main().catch((err) => {
    console.error(err);
});

