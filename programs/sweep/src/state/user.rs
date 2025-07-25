use anchor_lang::prelude::*;

#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub total_swept: u64,
    pub rewards_earned: u64,
}
