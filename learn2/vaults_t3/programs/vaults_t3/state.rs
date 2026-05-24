use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,

}
//we can directly use a macro initSpace to calculate the size of the account, so we don't need to implement the length function for the account struct
// impl VaultState {
//     pub const LEN: usize = core::mem::size_of::<VaultState>() + 8;
// }