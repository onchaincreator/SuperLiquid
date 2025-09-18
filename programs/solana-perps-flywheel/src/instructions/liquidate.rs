use crate::events::*;
use crate::math::current_mark_price_fp;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

pub fn liquidate(ctx: Context<Liquidate>) -> Result<()> {
    let m = &ctx.accounts.market;
    let cfg = &ctx.accounts.config;
    let mark_fp = current_mark_price_fp(m, &ctx.accounts.oracle)?;
    let up = &mut ctx.accounts.user_position;
    let notional_fp = (up.base_size.abs() as u128) * mark_fp / FP;
    let entry_fp = up.entry_price_fp as u128;
    let pnl_fp = if up.base_size >= 0 {
        (mark_fp as i128 - entry_fp as i128) * (up.base_size as i128) / FP as i128
    } else {
        (entry_fp as i128 - mark_fp as i128) * (-(up.base_size as i128)) / FP as i128
    };
    let equity_fp = (up.margin_deposited as i128) * FP as i128 + pnl_fp;
    let mm_req_fp = (notional_fp as u128 * (m.maintenance_margin_bps as u128)) / 10_000u128;

    if equity_fp < mm_req_fp as i128 {
        let liq_fee = ((notional_fp as u128) * (cfg.liq_fee_bps as u128) / 10_000) as u64;
        let seize = up.margin_deposited.min(liq_fee);
        token::transfer(ctx.accounts.transfer_vault_to_fee_dest(), seize)?;
        let remaining = up.margin_deposited.saturating_sub(seize);
        if remaining > 0 {
            token::transfer(ctx.accounts.transfer_vault_to_user(), remaining)?;
        }
        up.base_size = 0;
        up.margin_deposited = 0;
        emit!(Liquidated {
            user: up.owner,
            market: up.market,
            seized_collateral: seize
        });
    }
    Ok(())
}

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,
    #[account(mut)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub market: Account<'info, Market>,
    pub oracle: Account<'info, OraclePrice>,
    #[account(mut, seeds=[b"pos", user_position.owner.as_ref(), market.key().as_ref()], bump)]
    pub user_position: Account<'info, UserPosition>,
    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_token: Account<'info, TokenAccount>,
    /// CHECK
    #[account(mut)]
    pub fee_destination: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
impl<'info> Liquidate<'info> {
    pub fn transfer_vault_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.vault_token.to_account_info(),
                to: self.user_token.to_account_info(),
                authority: self.config.to_account_info(),
            },
        )
    }
    pub fn transfer_vault_to_fee_dest(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.vault_token.to_account_info(),
                to: self.fee_destination.to_account_info(),
                authority: self.config.to_account_info(),
            },
        )
    }
}
