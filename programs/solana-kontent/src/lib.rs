use anchor_lang::prelude::*;

declare_id!("4C9v8ZmgtEbx8dDGb7YDXrsLbNswU3HAzhqBCKzUn21N");

#[program]
pub mod solana_kontent {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
