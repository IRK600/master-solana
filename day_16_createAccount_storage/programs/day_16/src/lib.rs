use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("8Pq3HLvQULHh4jHC6uJRGMkdBhgsVo8oV8DKXkhhCgRR");

#[program]
pub mod day_16 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(init,
              payer=signer,
              space=size_of::<MyStorage>() + 8,
              seeds = [],
              bump)]
    pub my_storage: Account<'info, MyStorage>,
    
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyStorage {
    x: i64,
    y: i64
}
