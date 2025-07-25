use anchor_lang::prelude::*;
use crate::state::UserAccount;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ReinvestArgs {
    pub percentage: u8,
}

#[derive(Accounts)]
pub struct Reinvest<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Reinvest>, args: ReinvestArgs) -> Result<()> {
    msg!("Reinvesting {}% of SOL into microcoins...", args.percentage);
    Ok(())
}
