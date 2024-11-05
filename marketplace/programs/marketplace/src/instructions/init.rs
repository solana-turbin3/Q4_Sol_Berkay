use crate::state::Marketplace;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::error::MarketplaceError;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = Marketplace::INIT_SPACE,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        seeds = [b"treasury",marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>, // the treasury that will hold the fees

    #[account(
        init,
        payer=admin,
        seeds=[b"rewards",marketplace.key().as_ref()],
        bump,
        mint::decimals=6,
        mint::authority=marketplace
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>, //TODO: check why do we need that rewards_mint

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        require!(
            name.len() <= 32 && name.len() > 0,
            MarketplaceError::StringLengthInvalid
        );
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            name,
            fee,
            rewards_bump: bumps.rewards_mint,
            treasury_bump: bumps.treasury,
            bump: self.marketplace.bump,
        });
        Ok(())
    }
}
