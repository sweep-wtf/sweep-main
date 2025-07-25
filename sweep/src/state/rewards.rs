use anchor_lang::prelude::*;

#[account]
pub struct RewardPool {
    pub total_rewards: u64,
    pub last_distribution: i64,
}
