use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token, TokenAccount, Mint, Transfer, transfer},
    associated_token::AssociatedToken,
};

use crate::state::{singleton::Singleton, boardroom::BoardroomAccount};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct BoardroomDeposit<'info> {
    #[account(
        mut,
        seeds = [
            b"singleton"
        ],
        bump = singleton.bump
    )]
    pub singleton: Account<'info, Singleton>,

    #[account(
        mut,
        mint::authority = token_authority
    )]
    pub unit_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = unit_mint,
        associated_token::authority = signer
    )]
    pub unit_payer_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = unit_mint,
        associated_token::authority = token_authority
    )]
    pub unit_custody_account: Account<'info, TokenAccount>,

    /// CHECK: This account is not read or written
    #[account(
        seeds = [
            b"token-authority"
        ],
        bump
    )]
    pub token_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            b"boardroom-account",
            signer.key().as_ref()
        ],
        bump = boardroom_account.bump
    )]
    pub boardroom_account: Account<'info, BoardroomAccount>,

    pub signer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> BoardroomDeposit<'info> {
    pub fn transfer_tokens_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.unit_payer_account.to_account_info(),
            to: self.unit_custody_account.to_account_info(),
            authority: self.token_authority.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn boardroom_deposit_handler(ctx: Context<BoardroomDeposit>, amount: u64) -> Result<()> {
    let singleton = &mut ctx.accounts.singleton;
    let boardroom_account = &mut ctx.accounts.boardroom_account;

    if boardroom_account.only_frozen_or_locked() == false {
        return err!(ErrorCode::InvalidAccountStatus);
    }

    boardroom_account.increment_balance_of_staged(amount);
    singleton.increment_total_balance_of_staged(amount);
    
    transfer(ctx.accounts.transfer_tokens_ctx(), amount)?;

    Ok(())
}