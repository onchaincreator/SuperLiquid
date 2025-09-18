use anchor_lang::prelude::*;


#[error_code]
pub enum PerpsError {
#[msg("Leverage too high")] LeverageTooHigh,
#[msg("Insufficient margin")] InsufficientMargin,
#[msg("Position would exceed per-market max size")] MaxPositionExceeded,
#[msg("Unauthorized")] Unauthorized,
#[msg("Math overflow")] MathOverflow,
#[msg("Oracle price stale or invalid")] BadOracle,
}
