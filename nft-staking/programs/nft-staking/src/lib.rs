use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;

use errors::*;
use instructions::*;
use state::*;

declare_id!("Dfp5Q4UEhtHADpFchLUySUVuSzWz5STG9qQi5HxZXv3n");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user_account(&ctx.bumps)
    }

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .initialize_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>, token_mint: Pubkey, token_amount: u64) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>, token_amount: u64) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
