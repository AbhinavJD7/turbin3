# Solana Learning Notes

Solana uses parallel processing for read operations and sequential processing for write operations. This means if you have 100 transactions that all read from the same account, they can be processed in parallel. However, if they all try to write to the same account, they will be processed one after the other to avoid conflicts.

## Solana Account Structure

PDA (Program Derived Address) accounts do not have a private key, so they cannot sign transactions, but they can be used to store data and be owned by a program.

A typical Solana account structure looks something like this:

```rust
{
    key: number,
    lamports: number,
    data: Uint8Array,
    owner: PublicKey,
    is_executable: boolean
}
```

## Programs

If `is_executable` is true, then the account is a program account. This means it contains executable code and can be invoked by other programs.

- Programs are stateless; they only hold compiled code and do not hold any data.
- All program accounts are owned by loaders.

## Transactions

- A transaction includes all accounts that it will reference.
- It is atomic: if one instruction fails to execute, the entire transaction fails.
- A transaction structure consists of a message and signers.

```rust
{
  message: {
    instructions: Array<Instructions>,
    recent_blockhash: number,
    fee_payer: PublicKey,
  },
  signers: Array<Uint8Array>
}
```

## Compute

- All on-chain actions require compute units.
- Solana has a maximum number of compute units per block (e.g., 48 million or 60 million).
- Adding extra compute units is possible but generally not recommended.

## PDA (Program Derived Address)