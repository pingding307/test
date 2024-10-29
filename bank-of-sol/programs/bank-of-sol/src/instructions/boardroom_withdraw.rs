use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token, TokenAccount, Mint, Transfer, transfer},
    associated_token::AssociatedToken,
};
use crate::state::{singleton::Singleton, boardroom::BoardroomAccount};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct BoardroomWithdraw<'info> {
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
        seeds = [
            b"boardroom-account",
            signer.key().as_ref()
        ],
        bump = boardroom_account.bump
    )]
    pub boardroom_account: Account<'info, BoardroomAccount>,

    #[account(
        mut,
        mint::authority = token_authority
    )]
    pub unit_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = unit_mint,
        associated_token::authority = token_authority
    )]
    pub unit_custody_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = unit_mint,
        associated_token::authority = signer
    )]
    pub unit_receiver_account: Account<'info, TokenAccount>,

    /// CHECK: This account is not read or written
    #[account(
        seeds = [
            b"token-authority"
        ],
        bump
    )]
    pub token_authority: UncheckedAccount<'info>,

    pub signer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> BoardroomWithdraw<'info> {
    pub fn transfer_tokens_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.unit_custody_account.to_account_info(),
            to: self.unit_receiver_account.to_account_info(),
            authority: self.token_authority.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn boardroom_withdraw_handler(ctx: Context<BoardroomWithdraw>, amount: u64) -> Result<()> {
    let singleton = &mut ctx.accounts.singleton;
    let boardroom_account = &mut ctx.accounts.boardroom_account;
    let unit_auth_bump = singleton.unit_auth_bump;

    if boardroom_account.only_frozen_or_locked() == false {
        return err!(ErrorCode::InvalidAccountStatus);
    }

    boardroom_account.decrement_balance_of_staged(amount);
    singleton.decrement_total_balance_of_staged(amount);

    let unit_auth_seed = &[&b"token-authority"[..], &[unit_auth_bump]];

    transfer(ctx.accounts.transfer_tokens_ctx().with_signer(&[&unit_auth_seed[..]]), amount)?;

    Ok(())
}