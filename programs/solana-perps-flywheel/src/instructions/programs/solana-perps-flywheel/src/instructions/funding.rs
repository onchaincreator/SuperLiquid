use crate::math::current_mark_price_fp;
use crate::oracle::read_oracle_fp;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn settle_funding(ctx: Context<SettleFunding>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let m = &mut ctx.accounts.market;
    if now - m.last_funding_ts < 3600 {
        return Ok(());
    }
    let index_fp = read_oracle_fp(&ctx.accounts.oracle)?;
    let mark_fp = current_mark_price_fp(m, &ctx.accounts.oracle)?;
    let premium_fp = ((mark_fp as i128 - index_fp as i128) * (FP as i128)) / index_fp as i128;
    m.funding_rate_fp = premium_fp;
    m.last_funding_ts = now;
    Ok(())
}

#[derive(Accounts)]
pub struct SettleFunding<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    pub oracle: Account<'info, OraclePrice>,
}
