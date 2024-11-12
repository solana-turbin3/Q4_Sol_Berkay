
use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{transfer, Mint, MintTo, Token, TokenAccount, Transfer, TransferChecked}};

use crate::state::Config;

#[derive(Accounts)]
pub struct Deposit<'info>{
    
    #[account(mut)]
    pub user:Signer<'info>,
    pub mint_x:Account<'info,Mint>,
    pub mint_y:Account<'info,Mint>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config",config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config:Account<'info,Config>,
    #[account(
        seeds = [b"lp",config.key().as_ref()],
        bump = config.lp_bump
    )]
    pub mint_lp:Account<'info,Mint>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub vault_x:Account<'info,TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user
    )]
    pub vault_y:Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_x:Account<'info,TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user
    )]
    pub user_y:Account<'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user
    )]
    pub user_lp:Account<'info,TokenAccount>,
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(
        &mut self, 
        amount:u64, // amount of lp tokens that the user wants to "claim"
        max_x:u64, // max amount of x tokens that the user is willing to deposit
        max_y:u64, // max amount of y tokens that the user is willing to deposit
    ) -> Result<()> {
        require!(self.config.locked ==false, AmmError::PoolLocked);
        require!(amount !=, AmmError::InvalidAmount);

        let (x,y) = match self.mint_lp.supply == 0 && self.vault_x.amount == 0 && self.vault_y.amount == 0 {
            true => (max_x,max_y),
            false => {
                let amounts = ConstantProduct::xy_deposit_amounts_from_l(
                    x:self.vault_x.amount,
                    y:self.vault_y.amount,
                    l:self.mint_lp.supply,,
                    a:amount
                ).unwrap()
                (amounts.x,amounts.y)
            }
        };
        require!(x <= max_x && y<= max_y, AmmError::SlippageExceeded);

        // deposit token x 
        self.deposit_tokens(true,x)?;
        // deposit token y
        self.deposit_tokens(false,y)?;

        // mint lp tokens
        self.mint_lp_tokens(amount)?;
        Ok(())
    }

    pub fn deposit_tokens(&mut self, is_x:bool,amount:u64) -> Result<()> {
        let (from,to) = match is_x {
            true => (self.user_x.to_account_info(),self.vault_x.to_account_info());
            false => (self.user_y.to_account_info(),self.vault_y.to_account_info());

            let cpi_program = self.token_program.to_account_info();

            let cpi_accounts = Transfer {
                from,
                to,
                authority:self.user.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(cpi_program,cpi_accounts);
            transfer(cpi_ctx,amount)?;
            Ok(())
        }

        pub fn mint_lp_tokens(&self,amount:u64) -> Result<()> {
            let cpi_program = self.token_program.to_account_info();
            let cpi_accounts = MintTo {
                mint:self.mint_lp.to_account_info(),
                to:self.user_lp.to_account_info(),
                authority:self.config.to_account_info(),
            };

            let seeds = &[
                &b"config"[..],
                &[self.config.seed.to_le_bytes()],
                &[self.config.config_bump],
            ]

            let signer_seeds = [&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);
            mint_to(cpi_ctx,amount)?;
            Ok(())
        }
}