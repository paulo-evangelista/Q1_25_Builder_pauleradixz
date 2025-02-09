use anchor_lang::prelude::*;

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
    // The user account, which will pay for the initialization
    #[account(mut)]
    pub user: Signer<'info>,
    
    // The user account to be initialized, with specific space and seeds
    #[account(
        init,
        payer = user,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump,
        space = 8 + UserAccount::INIT_SPACE,
    )]
    pub user_account: Account<'info, UserAccount>,
    
    // System program required for account creation
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    // Function to initialize the user account
    pub fn initialize_user(&mut self, bumps: &InitializeBumps) -> Result<()> {
        // Set the inner state of the user account with initial values
        self.user_account.set_inner(UserAccount { 
            points: 0, 
            amount_staked: 0, 
            bump: bumps.user_account 
        });

        Ok(())
    }
}