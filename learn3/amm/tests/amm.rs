#![cfg(feature = "test-bpf")]
mod ix_handlers;

use anchor_lang::{
    prelude::{Pubkey, Rent},
    InstructionData, ToAccountMetas,
};
use anchor_spl::associated_token::get_associated_token_address;
use litesvm::{LiteSVM, TransactionResult};
use solana_sdk::{
    instruction::Instruction, signature::Keypair, signer::Signer, transaction::VersionedTransaction,
};

use ix_handlers::*;

fn send(
    svm: &mut LiteSVM,
    ix: Instruction,
    signers: &[&Keypair],
) -> TransactionResult {
    let mut tx =
        VersionedTransaction::try_new(vec![ix], signers).expect("Failed to create transaction");
    svm.send_transaction(tx)
}

fn setup() -> (
    LiteSVM,
    Keypair,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
    Pubkey,
) {
    let mut svm = LiteSVM::new();
    let program_id = amm::id();
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    // Create two mints and get their addresses
    let mint_x = svm.create_mint(&payer, None, 6);
    let mint_y = svm.create_mint(&payer, None, 6);

    let (config, _) = Pubkey::find_program_address(&[b"config", &123u64.to_le_bytes()], &program_id);
    let (mint_lp, _) = Pubkey::find_program_address(&[b"lp", config.as_ref()], &program_id);

    let vault_x = get_associated_token_address(&config, &mint_x);
    let vault_y = get_associated_token_address(&config, &mint_y);

    (
        svm, payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y,
    )
}

#[test]
fn test_initialize() {
    let (mut svm, payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y) = setup();
    let init_ix = create_initialize_ix(&payer, &mint_x, &mint_y, &config, &mint_lp, &vault_x, &vault_y);
    let res = send(&mut svm, init_ix, &[&payer]);
    assert!(res.is_ok());
}

#[test]
fn test_deposit() {
    let (mut svm, payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y) = setup();
    let init_ix = create_initialize_ix(&payer, &mint_x, &mint_y, &config, &mint_lp, &vault_x, &vault_y);
    send(&mut svm, init_ix, &[&payer]).unwrap();

    let deposit_ix = create_deposit_ix(&payer, &mint_x, &mint_y, &config, &mint_lp, &vault_x, &vault_y);
    let res = send(&mut svm, deposit_ix, &[&payer]);
    assert!(res.is_ok());
}

#[test]
fn test_withdraw() {
    let (mut svm, payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y) = setup();
    let init_ix = create_initialize_ix(&payer, &mint_x, &mint_y, &config, &mint_lp, &vault_x, &vault_y);
    send(&mut svm, init_ix, &[&payer]).unwrap();

    let deposit_ix = create_deposit_ix(&payer, &mint_x, &mint_y, &config, &mint_lp, &vault_x, &vault_y);
    send(&mut svm, deposit_ix, &[&payer]).unwrap();

    let withdraw_ix = create_withdraw_ix(&payer, &mint_x, &mint_y, &config, &mint_lp, &vault_x, &vault_y);
    let res = send(&mut svm, withdraw_ix, &[&payer]);
    assert!(res.is_ok());
}

#[test]
fn test_swap() {
    let (mut svm, payer, mint_x, mint_y, config, mint_lp, vault_x, vault_y) = setup();
    let init_ix = create_initialize_ix(&payer, &mint_x, &mint_y, &config, &mint_lp, &vault_x, &vault_y);
    send(&mut svm, init_ix, &[&payer]).unwrap();

    let deposit_ix = create_deposit_ix(&payer, &mint_x, &mint_y, &config, &mint_lp, &vault_x, &vault_y);
    send(&mut svm, deposit_ix, &[&payer]).unwrap();

    let swap_ix = create_swap_ix(&payer, &mint_x, &mint_y, &config, &vault_x, &vault_y);
    let res = send(&mut svm, swap_ix, &[&payer]);
    assert!(res.is_ok());
}
