use anchor_lang::{prelude::*, InstructionData, ToAccountMetas};
use solana_sdk::{instruction::Instruction, signature::Keypair};

pub fn create_deposit_ix(
    payer: &Keypair,
    mint_x: &Pubkey,
    mint_y: &Pubkey,
    config: &Pubkey,
    mint_lp: &Pubkey,
    vault_x: &Pubkey,
    vault_y: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: amm::id(),
        accounts: amm::accounts::Deposit {
            user: payer.pubkey(),
            mint_x: *mint_x,
            mint_y: *mint_y,
            config: *config,
            mint_lp: *mint_lp,
            vault_x: *vault_x,
            vault_y: *vault_y,
            user_x: anchor_spl::associated_token::get_associated_token_address(
                &payer.pubkey(),
                mint_x,
            ),
            user_y: anchor_spl::associated_token::get_associated_token_address(
                &payer.pubkey(),
                mint_y,
            ),
            user_lp: anchor_spl::associated_token::get_associated_token_address(
                &payer.pubkey(),
                mint_lp,
            ),
            token_program: anchor_spl::token::ID,
            system_program: anchor_lang::system_program::ID,
            associated_token_program: anchor_spl::associated_token::ID,
        }
        .to_account_metas(None),
        data: amm::instruction::Deposit {
            amount: 100_000_000,
            max_x: 100_000_000,
            max_y: 100_000_000,
        }
        .data(),
    }
}
