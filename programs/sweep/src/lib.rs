use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;

declare_id!("SWEEP1111111111111111111111111111111111111");

#[program]
pub mod sweep {
    use super::*;

    pub fn sweep_dust(ctx: Context<SweepDust>, args: SweepArgs) -> Result<()> {
        sweep_dust::handler(ctx, args)
    }

    pub fn reinvest(ctx: Context<Reinvest>, args: ReinvestArgs) -> Result<()> {
        reinvest::handler(ctx, args)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        claim_rewards::handler(ctx)
    }
}
