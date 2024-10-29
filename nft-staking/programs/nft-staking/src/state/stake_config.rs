use anchor_lang::prelude::*;
// that config is required to set some config to the stake operation

#[account]
pub struct StakeConfig {
    pub points_per_stake: u8,
    pub max_stake: u8,      // max number a user can stake
    pub freeze_period: u32, // number of days a user can't withdraw
    pub rewards_bump: u8,
    pub stake_bump: u8, // since our stake config is a pda, we need to store that bump
}

impl Space for StakeConfig {
    const INIT_SPACE: usize = 8 + 1 + 1 + 4 + 1 + 1;
}
