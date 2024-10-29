use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{approve, Approve, ApproveChecked, Mint, Token, TokenAccount},
};

use crate::{StakeAccount, StakeConfig, StakeError, UserAccount};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>, // the nft that the user wants to stake
    pub collection_mint: Account<'info, Mint>, // the collection of the nft that the user wants to stake

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub mint_ata: Account<'info, TokenAccount>, // the associated token account of the nft that the user wants to stake

    #[account(
        seeds = [b"metadata".as_ref(),metadata_program.key().as_ref(),mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref()==collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>, // that account is on created on the metadata program actually, that's why we need to add its seeds

    #[account(
        seeds = [b"metadata".as_ref(),metadata_program.key().as_ref(),mint.key().as_ref(),b"edition"],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>, // that will verify the mint supply is 1 and the decimals are 0 on the mint account. Mint authority is passed to the MasterEdition account

    #[account(
        seeds = [b"config"],
        bump=config_account.bump,
    )]
    pub config_account: Account<'info, StakeConfig>,

    #[account(
        init,
        payer = user,
        seeds=[b"stake",config_account.key().as_ref(),mint.key().as_ref()],
        bump,
        space = StakeAccount::INIT_SPACE,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    pub metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        require!(
            self.user_account.amount_staked < self.config_account.max_stake,
            StakeError::MaxStakeReached
        );

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Approve {
            to: self.mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        approve(cpi_ctx, 1)?; // the authority will be delegated to the stake account, then the stake account can sign the freeze delegated account cpi

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.master_edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata = &self.metadata_program.to_account_info();

        let seeds = &[
            b"stake",
            self.config_account.to_account_info().key.as_ref(),
            self.mint.to_account_info().key.as_ref(),
            &[self.stake_account.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        FreezeDelegatedAccountCpi::new(
            metadata,
            FreezeDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            },
        )
        .invoke_signed(signer_seeds)?;

        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            mint: self.mint.key(),
            staked_at: Clock::get()?.unix_timestamp,
            bump: bumps.stake_account,
        });

        self.user_account.amount_staked += 1;
        Ok(())
    }
}
