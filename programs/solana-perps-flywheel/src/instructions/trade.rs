use anchor_lang::prelude::*;
use crate::state::*;


pub fn open_position(ctx: Context<OpenPosition>, is_long: bool, quote_to_spend: u64, leverage_x: u16) -> Result<()> {
    let cfg = &ctx.accounts.config;
    require!(!cfg.paused, PerpsError::Unauthorized);
    require!(leverage_x as u64 <= MAX_LEVERAGE_X, PerpsError::LeverageTooHigh);
    require!(leverage_x <= ctx.accounts.market.taker_leverage_cap_x, PerpsError::LeverageTooHigh);


    let margin = (quote_to_spend as u128 / leverage_x as u128) as u64;
    token::transfer(ctx.accounts.transfer_user_to_vault(), margin)?;


    let price_fp = current_mark_price_fp(&ctx.accounts.market, &ctx.accounts.oracle)?;
    let base_size_fp: u128 = (quote_to_spend as u128 * FP) / price_fp;
    let base_size_units: u64 = (base_size_fp / FP) as u64;
    require!(base_size_units <= ctx.accounts.market.max_position_base, PerpsError::MaxPositionExceeded);


    let up = &mut ctx.accounts.user_position;
    up.owner = ctx.accounts.user.key();
    up.market = ctx.accounts.market.key();
    up.is_long = is_long;
    up.base_size = if is_long { base_size_units as i64 } else { -(base_size_units as i64) };
    up.entry_price_fp = price_fp;
    up.margin_deposited = margin;
    up.last_funding_settled = Clock::get()?.unix_timestamp;


    emit!(PositionOpened { user: up.owner, market: up.market, is_long, base_size: base_size_units, entry_price_fp: price_fp });
    Ok(())
}


pub fn close_position(ctx: Context<ClosePosition>) -> Result<()> {
    let mark_fp = current_mark_price_fp(&ctx.accounts.market, &ctx.accounts.oracle)?;
    let up = &mut ctx.accounts.user_position;
    let signed_base = up.base_size as i128;
    let entry_fp = up.entry_price_fp as i128;
    let notional_entry_fp = (signed_base.abs() as i128) * entry_fp;
    let notional_exit_fp = (signed_base.abs() as i128) * (mark_fp as i128);
    let direction = if signed_base >= 0 { 1 } else { -1 };
    let pnl_fp: i128 = direction * (notional_exit_fp - notional_entry_fp) / FP as i128;


    let fee_fp: u128 = (notional_exit_fp.unsigned_abs() * (ctx.accounts.config.fee_bps as u128)) / 10_000u128;
    let mut settle_fp: i128 = up.margin_deposited as i128 * FP as i128 + pnl_fp - fee_fp as i128;
    if settle_fp < 0 { settle_fp = 0; }
    let settle_amt: u64 = (settle_fp as u128 / FP) as u64;


    token::transfer(ctx.accounts.transfer_vault_to_user(), settle_amt)?;
    let fee_amt: u64 = (fee_fp / FP) as u64;
    token::transfer(ctx.accounts.transfer_vault_to_fee_dest(), fee_amt)?;


    emit!(PositionClosed { user: up.owner, market: up.market, pnl_fp, fees_fp: fee_fp });
    up.base_size = 0;
    up.margin_deposited = 0;
    Ok(())
}


#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)] pub user: Signer<'info>,
    #[account(mut)] pub config: Account<'info, Config>,
    #[account(mut)] pub market: Account<'info, Market>,
    pub oracle: Account<'info, OraclePrice>,
    #[account(init_if_needed, payer = user, space = 8 + 200, seeds=[b"pos", user.key().as_ref(), market.key().as_ref()], bump
    )]
    pub user_position: Account<'info, UserPosition>,
    #[account(mut)] pub user_token: Account<'info, TokenAccount>,
    #[account(mut)] pub vault_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
impl<'info> OpenPosition<'info> {
    pub fn transfer_user_to_vault(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(self.token_program.to_account_info(), Transfer { from: self.user_token.to_account_info(), to: self.vault_token.to_account_info(), authority: self.user.to_account_info() })
    }
}


#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(mut)] pub user: Signer<'info>,
    #[account(mut)] pub config: Account<'info, Config>,
    #[account(mut)] pub market: Account<'info, Market>,
    pub oracle: Account<'info, OraclePrice>,
    #[account(mut, seeds=[b"pos", user.key().as_ref(), market.key().as_ref()], bump
    )] pub user_position: Account<'info, UserPosition>,
    #[account(mut)] pub user_token: Account<'info, TokenAccount>,
    #[account(mut)] pub vault_token: Account<'info, TokenAccount>,
    /// CHECK: fees go to Pump/Pumpswap LP token account
    #[account(mut)] pub fee_destination: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
impl<'info> ClosePosition<'info> {
    pub fn transfer_vault_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(self.token_program.to_account_info(), Transfer { from: self.vault_token.to_account_info(), to: self.user_token.to_account_info(), authority: self.config.to_account_info() })
    }
    pub fn transfer_vault_to_fee_dest(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(self.token_program.to_account_info(), Transfer { from: self.vault_token.to_account_info(), to: self.fee_destination.to_account_info(), authority: self.config.to_account_info() })
    }
