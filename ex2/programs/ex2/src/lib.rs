use anchor_lang::prelude::*;

declare_id!("C4pWBtp5XttdUfeDqYu3gLhpuPardmZKXtFiXK7wk86i");

#[program]
pub mod ex2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
