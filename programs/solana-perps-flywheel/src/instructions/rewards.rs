use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

pub fn sweep_creator_rewards(ctx: Context<SweepCreatorRewards>, amount: u64) -> Result<()> {
    let cfg = &ctx.accounts.config;
    require_keys_eq!(
        cfg.creator_reward_mint,
        ctx.accounts.creator_reward_mint.key()
    );
    token::transfer(
        ctx.accounts.transfer_rewards_to_fee_dest(),
        amount * (cfg.creator_reward_bps as u64) / 10_000,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct SweepCreatorRewards<'info> {
    pub config: Account<'info, Config>,
    pub creator_reward_mint: Account<'info, Mint>,
    #[account(mut)]
    pub creator_reward_source: Account<'info, TokenAccount>,
    /// CHECK
    #[account(mut)]
    pub fee_destination: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
impl<'info> SweepCreatorRewards<'info> {
    pub fn transfer_rewards_to_fee_dest(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.creator_reward_source.to_account_info(),
                to: self.fee_destination.to_account_info(),
                authority: self.config.to_account_info(),
            },
        )
    }
}
