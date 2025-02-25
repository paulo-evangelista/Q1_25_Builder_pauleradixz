use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};
use crate::state::config::Config;

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // Conta de token de pagamento do usuário
    #[account(mut)]
    pub payment_source: Box<Account<'info, TokenAccount>>,

    // Conta que receberá o pagamento (tesouraria)
    #[account(mut)]
    pub treasury: Box<Account<'info, TokenAccount>>,

    // Mint do token que será emitido
    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,

    // Conta destino dos tokens mintados
    #[account(mut)]
    pub recipient_token_account: Box<Account<'info, TokenAccount>>,

    // Autoridade do mint (pode ser um PDA; se for o caso, use seeds e bump)
    pub mint_authority: Signer<'info>,

    // Configuração do programa
    #[account(mut, has_one = owner)]
    pub config: Box<Account<'info, Config>>,

    pub owner: Signer<'info>,

    // Mint do token de pagamento
    pub payment_mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

    pub fn mint_tokens(ctx: Context<MintTokens>, mint_amount: u64) -> Result<()> {
        
        // Verifica se o valor a ser mintado é menor ou igual ao máximo permitido
        
        
        let cost = ctx.accounts.config.price.checked_mul(mint_amount)
        .ok_or(ErrorCode::MathOverflow)?;
    
    // Transferência do token de pagamento
    let cpi_accounts = Transfer {
        from: ctx.accounts.payment_source.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, cost)?;
    
    // Mint dos tokens
    let cpi_accounts = MintTo {
        mint: ctx.accounts.token_mint.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::mint_to(cpi_ctx, mint_amount)?;
    
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Math operation overflow")]
    MathOverflow,
}