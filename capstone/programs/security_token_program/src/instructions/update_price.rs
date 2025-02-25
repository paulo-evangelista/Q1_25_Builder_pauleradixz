use anchor_lang::prelude::*;
use crate::state::config::Config;

pub fn update_price(ctx: Context<UpdatePrice>, new_price: u64) -> Result<()> {
    ctx.accounts.config.price = new_price;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut, has_one = owner)]
    pub config: Box<Account<'info, Config>>,
    pub owner: Signer<'info>,
}
