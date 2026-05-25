use anchor_lang::{prelude::*, InstructionData, ToAccountMetas};
use solana_sdk::{instruction::Instruction, signature::Keypair};

pub fn create_initialize_ix(
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
        accounts: amm::accounts::Initialize {
            user: payer.pubkey(),
            mint_x: *mint_x,
            mint_y: *mint_y,
            config: *config,
            mint_lp: *mint_lp,
            vault_x: *vault_x,
            vault_y: *vault_y,
            token_program: anchor_spl::token::ID,
            system_program: anchor_lang::system_program::ID,
            associated_token_program: anchor_spl::associated_token::ID,
        }
        .to_account_metas(None),
        data: amm::instruction::Initialize {
            seed: 123,
            fee: 30,
        }
        .data(),
    }
}
