use anchor_lang::prelude::*;
use crate::state::{Market, FP};
use crate::oracle::read_oracle_fp;


pub fn current_mark_price_fp(m: &Account<Market>, oracle: &Account<crate::state::OraclePrice>) -> Result<u128> {
let index_fp = read_oracle_fp(oracle)?;
let k = m.skew_k_bps as i128; // basis points skew strength
let ratio_fp = (m.amm_quote_reserve_fp as i128 * FP as i128) / m.amm_base_reserve_fp as i128;
let skew_term_fp = (k * (ratio_fp - FP as i128)) / 10_000i128;
let mark_fp = ((index_fp as i128) + ((index_fp as i128 * skew_term_fp) / FP as i128)) as u128;
Ok(mark_fp.max(1))
}
