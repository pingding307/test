use anchor_lang::prelude::*;

use crate::state::boardroom::BoardroomAccount;
use crate::state::singleton::Singleton;

#[derive(Accounts)]
pub struct CreateBoardroomAccount<'info> {
    #[account(
        init,
        payer = signer,
        space = BoardroomAccount::LEN,
        seeds = [
            b"boardroom-account",
            signer.key().as_ref()
        ],
        bump
    )]
    pub boardroom_account: Account<'info, BoardroomAccount>,

    #[account(
        seeds = [
            b"singleton"
        ],
        bump = singleton.bump
    )]
    pub singleton: Account<'info, Singleton>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_boardroom_account_handler(ctx: Context<CreateBoardroomAccount>) -> Result<()> {
    let boardroom_account = &mut ctx.accounts.boardroom_account;
    let singleton = &ctx.accounts.singleton;

    **boardroom_account = BoardroomAccount::init(singleton.epoch(), ctx.bumps.boardroom_account);

    Ok(())
}