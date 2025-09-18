use crate::errors::PerpsError;
use crate::state::*;
use anchor_lang::prelude::*;

pub fn create_market(
    ctx: Context<CreateMarket>,
    symbol: [u8; 12],
    base_decimals: u8,
    skew_k_bps: u32,
    max_position_base: u64,
    maintenance_margin_bps: u16,
    taker_leverage_cap_x: u16,
    amm_base_reserve_fp: u128,
    amm_quote_reserve_fp: u128,
) -> Result<()> {
    require!(
        taker_leverage_cap_x as u64 <= MAX_LEVERAGE_X,
        PerpsError::LeverageTooHigh
    );
    let m = &mut ctx.accounts.market;
    m.symbol = symbol;
    m.base_decimals = base_decimals;
    m.oracle = ctx.accounts.oracle.key();
    m.skew_k_bps = skew_k_bps;
    m.max_position_base = max_position_base;
    m.maintenance_margin_bps = maintenance_margin_bps;
    m.taker_leverage_cap_x = taker_leverage_cap_x;
    m.amm_base_reserve_fp = amm_base_reserve_fp;
    m.amm_quote_reserve_fp = amm_quote_reserve_fp;
    m.funding_rate_fp = 0;
    m.last_funding_ts = Clock::get()?.unix_timestamp;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    #[account(init, payer = payer, space = 8 + 256)]
    pub market: Account<'info, Market>,
    pub oracle: Account<'info, OraclePrice>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
