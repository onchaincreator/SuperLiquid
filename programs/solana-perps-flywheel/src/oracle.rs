use crate::errors::PerpsError;
use crate::state::OraclePrice;
use anchor_lang::prelude::*;

pub fn read_oracle_fp(oracle: &Account<OraclePrice>) -> Result<u128> {
    let now = Clock::get()?.unix_timestamp;
    require!(now - oracle.last_updated_ts < 120, PerpsError::BadOracle);
    Ok(oracle.price_fp)
}
