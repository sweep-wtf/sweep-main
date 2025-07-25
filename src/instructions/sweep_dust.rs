use anchor_lang::prelude::*;
use crate::state::{UserAccount, Config};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SweepArgs {
    pub min_out_sol: u64,
}

#[derive(Accounts)]
pub struct SweepDust<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SweepDust>, args: SweepArgs) -> Result<()> {
    msg!("Sweeping dust tokens...");
    Ok(())
}
