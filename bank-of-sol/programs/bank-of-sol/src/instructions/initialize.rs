use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, SetAuthority, set_authority, spl_token::instruction::AuthorityType},
    associated_token::AssociatedToken,
};

use crate::{state::singleton::Singleton, types::epoch::Epoch};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = creator,
        space = Singleton::LEN,
        seeds = [
            b"singleton"
        ],
        bump
    )]
    pub singleton: Account<'info, Singleton>,

    #[account(
        mint::decimals = 0,
    )]
    pub collection: Account<'info, Mint>,

    pub raydium_pool: AccountInfo<'info>,

    #[account(
        mut,
        mint::authority = creator
    )]
    pub unit_mint: Account<'info, Mint>,

    #[account(
        seeds = [
            b"token-authority"
        ],
        bump
    )]
    pub token_authority: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = creator,
        associated_token::mint = unit_mint,
        associated_token::authority = token_authority
    )]
    pub unit_custody: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = creator,
        associated_token::mint = unit_usdc_lp_mint,
        associated_token::authority = token_authority
    )]
    pub unit_usdc_lp_custody: Box<Account<'info, TokenAccount>>,

    pub unit_usdc_lp_mint: AccountInfo<'info>,

    #[account(mut)]
    pub creator: Signer<'info>,

    /// CHECK: This account is not read or written
    pub nft_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn transfer_auth_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.unit_mint.to_account_info(),
            current_authority: self.creator.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn initialize_handler(ctx: Context<Initialize>, minimum_period: i64) -> Result<()> {
    require_gte!(minimum_period, 0, ErrorCode::NegativePeriodValue);

    let token_authority = ctx.accounts.token_authority.key();
    
    set_authority(
        ctx.accounts.transfer_auth_ctx(),
        AuthorityType::MintTokens,
        Some(token_authority)
    )?;

    let singleton = &mut ctx.accounts.singleton;

    **singleton = Singleton::init(
        ctx.accounts.creator.key(),
        ctx.accounts.raydium_pool.key(),
        Epoch::init(),
        ctx.accounts.unit_mint.key(),
        ctx.accounts.token_authority.key(),
        ctx.accounts.unit_custody.key(),
        ctx.accounts.unit_usdc_lp_custody.key(),
        ctx.bumps.token_authority,
        ctx.bumps.singleton
    );

    Ok(())
}