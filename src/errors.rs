use anchor_lang::prelude::*;

#[error_code]
pub enum SweepError {
    #[msg("Insufficient dust to sweep.")]
    InsufficientDust,
    #[msg("Unauthorized.")]
    Unauthorized,
}
