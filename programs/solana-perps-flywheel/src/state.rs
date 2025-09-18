use anchor_lang::prelude::*;


pub const FP: u128 = 1_000_000; // fixed point 1e6
pub const MAX_LEVERAGE_X: u64 = 40;


#[account]
pub struct Config {
pub admin: Pubkey,
pub quote_mint: Pubkey,
pub fee_bps: u16,
pub liq_fee_bps: u16,
pub fee_destination: Pubkey, // SPL token account (set to Pump/Pumpswap LP vault)
pub insurance_vault: Pubkey,
pub creator_reward_mint: Pubkey,
pub creator_reward_bps: u16,
pub paused: bool,
}


#[account]
pub struct Market {
pub symbol: [u8; 12],
pub base_decimals: u8,
pub oracle: Pubkey,
pub amm_base_reserve_fp: u128,
pub amm_quote_reserve_fp: u128,
pub funding_rate_fp: i128,
pub last_funding_ts: i64,
pub skew_k_bps: u32,
pub max_position_base: u64,
pub maintenance_margin_bps: u16,
pub taker_leverage_cap_x: u16,
}


#[account]
pub struct UserPosition {
pub owner: Pubkey,
pub market: Pubkey,
pub is_long: bool,
pub base_size: i64,
pub entry_price_fp: u128,
pub margin_deposited: u64,
pub last_funding_settled: i64,
}


#[account]
pub struct OraclePrice { pub price_fp: u128, pub last_updated_ts: i64 }
