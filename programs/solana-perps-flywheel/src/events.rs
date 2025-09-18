use anchor_lang::prelude::*;


#[event]
pub struct PositionOpened { pub user: Pubkey, pub market: Pubkey, pub is_long: bool, pub base_size: u64, pub entry_price_fp: u128 }
#[event]
pub struct PositionClosed { pub user: Pubkey, pub market: Pubkey, pub pnl_fp: i128, pub fees_fp: u128 }
#[event]
pub struct Liquidated { pub user: Pubkey, pub market: Pubkey, pub seized_collateral: u64 }
