# RAI Token Turbin3 Assignmnet 2

This project contains two scripts to demonstrate how to work with SPL tokens and NFTs on the Solana blockchain.

## Prerequisites

- Node.js and npm installed
- A Solana wallet with some devnet SOL. You can get some from a faucet.
- A `devnet-wallet.json` file in the root of the project containing your wallet's secret key.

## Installation

1.  Clone the repository.
2.  Navigate to the `rai_token` directory.
3.  Install the dependencies:

```bash
npm install
```

## Scripts

### Mint SPL Token

This script creates a new SPL token and mints 1 token to your account.

To run the script:

```bash
npx ts-node src/spl/spl_init.ts
```

### Mint NFT

This script mints a new NFT using the Metaplex MPL Core standard.

To run the script:

```bash
npx ts-node src/spl/spl_metadata.ts
```
