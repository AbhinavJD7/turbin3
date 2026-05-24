use anchor_lang::prelude::*;
use crate::state::VaultState;

// Explanation: We have a signer here, it is the one who wants to create this vault.
// Then we have a state account, which is a PDA account - it stores the state of the vault,
// initialized in this instruction with the user as payer.
// Then we have a vault account, also a PDA, that stores the funds. Its seeds are derived
// from the state account so it can always be found given the state PDA.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = 8 + VaultState::INIT_SPACE,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        // Save bump seeds to the state account
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}
