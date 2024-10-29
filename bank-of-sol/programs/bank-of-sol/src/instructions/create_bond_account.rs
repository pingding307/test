use anchor_lang::prelude::*;

use crate::state::bond_account::{BondAccount, BondAccountsStore};
use crate::state::singleton::Singleton;

#[derive(Accounts)]
pub struct CreateBondAccount<'info> {
    #[account(
        init,
        payer = signer,
        space = BondAccount::LEN,
        seeds = [
            b"bond-account",
            signer.key().as_ref(),
            (bond_accounts_store.number_of_accounts + 1).to_be_bytes().as_ref()
        ],
        bump
    )]
    pub bond_account: Account<'info, BondAccount>,

    #[account(
        mut,
        seeds = [
            b"bond-accounts-store",
            signer.key().as_ref()
        ],
        bump = bond_accounts_store.bump
    )]
    pub bond_accounts_store: Account<'info, BondAccountsStore>,

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

pub fn create_bond_account_handler(ctx: Context<CreateBondAccount>, amount: u64) -> Result<()> {
    let bond_account = &mut ctx.accounts.bond_account;
    let bond_accounts_store = &mut ctx.accounts.bond_accounts_store;
    let singleton = &ctx.accounts.singleton;
    let bump = ctx.bumps.bond_account;

    **bond_account = BondAccount::init(
        ctx.accounts.signer.key(),
        bump,
        amount,
        singleton.epoch.index,
        singleton.interest_rate(),
        singleton.bonds.total_bonds_purchased,
    );

    bond_accounts_store.increment();

    Ok(())
}