use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::state::{StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Claim<'info> {
    // The user who is claiming rewards
    #[account(mut)]
    pub user: Signer<'info>,
    
    // The user's account, used to track their staking information
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    
    // The mint account for the rewards token
    #[account(
        mut,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump = config.rewards_bump
    )]
    pub rewards_mint: Account<'info, Mint>,
    
    // The configuration account for staking
    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,
    
    // The associated token account for the user's rewards
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user,
    )]
    pub rewards_ata: Account<'info, TokenAccount>,
    
    // System program required for account creation
    pub system_program: Program<'info, System>,
    
    // Token program required for minting tokens
    pub token_program: Program<'info, Token>,
    
    // Associated token program for creating associated token accounts
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Claim<'info> {
    // Function to claim rewards
    pub fn claim(&mut self) -> Result<()> {
        // Get the account info for the token program
        let cpi_program = self.token_program.to_account_info();

        // Define the seeds for signing the transaction
        let seeds = &[
            b"config".as_ref(),
            &[self.config.bump]
        ];     
        let signer_seeds = &[&seeds[..]];

        // Set up the accounts required for the mint_to CPI call
        let cpi_accounts = MintTo {
            mint: self.rewards_mint.to_account_info(),
            to: self.rewards_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        // Create the CPI context with the signer seeds
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Mint the rewards to the user's associated token account
        mint_to(cpi_context, self.user_account.points as u64 * 10_u64.pow(self.rewards_mint.decimals as u32))?;

        // Reset the user's points after claiming
        self.user_account.points = 0;
        
        Ok(())
    }
}