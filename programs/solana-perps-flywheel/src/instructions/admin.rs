use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

pub fn initialize_config(
    ctx: Context<InitializeConfig>,
    fee_bps: u16,
    liq_fee_bps: u16,
    creator_reward_bps: u16,
) -> Result<()> {
    let cfg = &mut ctx.accounts.config;
    cfg.admin = ctx.accounts.admin.key();
    cfg.quote_mint = ctx.accounts.quote_mint.key();
    cfg.fee_bps = fee_bps;
    cfg.liq_fee_bps = liq_fee_bps;
    cfg.fee_destination = ctx.accounts.fee_destination.key();
    cfg.insurance_vault = ctx.accounts.insurance_vault.key();
    cfg.creator_reward_mint = ctx.accounts.creator_reward_mint.key();
    cfg.creator_reward_bps = creator_reward_bps;
    cfg.paused = false;
    Ok(())
}

pub fn set_fee_destination(ctx: Context<AdminOnly>, new_fee_dest: Pubkey) -> Result<()> {
    ctx.accounts.config.fee_destination = new_fee_dest;
    Ok(())
}
pub fn edit_max_position(ctx: Context<AdminOnlyMarket>, new_max_base: u64) -> Result<()> {
    ctx.accounts.market.max_position_base = new_max_base;
    Ok(())
}
pub fn pause(ctx: Context<AdminOnly>, paused: bool) -> Result<()> {
    ctx.accounts.config.paused = paused;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(init, payer = admin, space = 8 + 256)]
    pub config: Account<'info, Config>,
    pub quote_mint: Account<'info, Mint>,
    /// CHECK: SPL token account for fee destination (Pump/Pumpswap LP vault)
    pub fee_destination: AccountInfo<'info>,
    pub insurance_vault: Account<'info, TokenAccount>,
    pub creator_reward_mint: Account<'info, Mint>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdminOnly<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, Config>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct AdminOnlyMarket<'info> {
    pub admin: Signer<'info>,
    #[account(mut)]
    pub market: Account<'info, Market>,
}
