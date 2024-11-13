use anchor_lang::prelude::*;

mod instructions;
mod state;

declare_id!("CBWE1L8fiRujHUAETBfuwCfnyyBXwW62GDWLcAPgSAQq");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
