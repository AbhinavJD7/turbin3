use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use vaults_t3; // Your program's crate name

// Helper function to set up the test environment
async fn setup() -> (LiteSVM, Keypair) {
    let program_id = vaults_t3::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = solana_program::borsh::try_to_vec(&vaults_t3::entry).unwrap();
    let mut program_test = ProgramTest::new(
        "vaults_t3",
        vaults_t3::ID,
        processor!(vaults_t3::entry),
    );

    let mut context = program_test.start_with_context().await;
    let payer = context.payer.insecure_clone();

    (context, payer)
}

#[tokio::test]
async fn test_initialize_deposit_withdraw_close() {
    let (_context, _payer) = setup().await;

    // Your test logic from the tutorial will go here.
    
    println!("Rust test file is set up. Ready for your test logic!");
}
