use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub owner: Pubkey,
    pub price: u64,
    pub max_supply: u64,
}