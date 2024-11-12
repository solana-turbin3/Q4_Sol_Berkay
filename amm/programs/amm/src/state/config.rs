
use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub seed: u64, // Seed to be able to create different pools / configs
    pub authority: Option<Pubkey>, // if we want an authority to lock the config account
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee:u16, // swap fee in basis points
    pub locked:bool, // if the pool is locked
    pub config_bump:u8, // bumps seed for the config account
    pub lp_bump:u8, // bumps seed for the lp account (liquidity provider)
}


impl Space for Config {
    const INIT_SPACE: usize = 8 + 8 + (1 + 32) + 32 + 32 + 2 + 1 + 1 + 1;
}