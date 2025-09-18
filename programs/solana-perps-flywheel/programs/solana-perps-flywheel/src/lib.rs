use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod math;
pub mod oracle;
pub mod state;

use instructions::*;

// Program ID
declare_id!("Fg1WhEel111111111111111111111111111111111");

#[program]
pub mod solana_perps_flywheel {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        fee_bps: u16,
        liq_fee_bps: u16,
        creator_reward_bps: u16,
    ) -> Result<()> {
        instructions::admin::initialize_config(ctx, fee_bps, liq_fee_bps, creator_reward_bps)
    }
    pub fn set_fee_destination(ctx: Context<AdminOnly>, new_fee_dest: Pubkey) -> Result<()> {
        instructions::admin::set_fee_destination(ctx, new_fee_dest)
    }
    pub fn pause(ctx: Context<AdminOnly>, paused: bool) -> Result<()> {
        instructions::admin::pause(ctx, paused)
    }

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
        instructions::create_market::create_market(
            ctx,
            symbol,
            base_decimals,
            skew_k_bps,
            max_position_base,
            maintenance_margin_bps,
            taker_leverage_cap_x,
            amm_base_reserve_fp,
            amm_quote_reserve_fp,
        )
    }

    pub fn edit_max_position(ctx: Context<AdminOnlyMarket>, new_max_base: u64) -> Result<()> {
        instructions::admin::edit_max_position(ctx, new_max_base)
    }

    pub fn open_position(
        ctx: Context<OpenPosition>,
        is_long: bool,
        quote_to_spend: u64,
        leverage_x: u16,
    ) -> Result<()> {
        instructions::trade::open_position(ctx, is_long, quote_to_spend, leverage_x)
    }

    pub fn close_position(ctx: Context<ClosePosition>) -> Result<()> {
        instructions::trade::close_position(ctx)
    }

    pub fn liquidate(ctx: Context<Liquidate>) -> Result<()> {
        instructions::liquidate::liquidate(ctx)
    }

    pub fn settle_funding(ctx: Context<SettleFunding>) -> Result<()> {
        instructions::funding::settle_funding(ctx)
    }

    pub fn sweep_creator_rewards(ctx: Context<SweepCreatorRewards>, amount: u64) -> Result<()> {
        instructions::rewards::sweep_creator_rewards(ctx, amount)
    }
}
