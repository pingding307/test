use anchor_lang::prelude::*;
use crate::state::bond_account::BondAccountsStore;

#[derive(Accounts)]
pub struct CreateBondAccountsStore<'info> {
    #[account(
        init,
        payer = signer,
        space = BondAccountsStore::LEN,
        seeds = [
            b"bond-accounts-store",
            signer.key().as_ref()
        ],
        bump
    )]
    pub bond_accounts_store: Account<'info, BondAccountsStore>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_bond_accounts_store_handler(ctx: Context<CreateBondAccountsStore>) -> Result<()> {
    let bump = ctx.bumps.bond_accounts_store;
    let bond_accounts_store = &mut ctx.accounts.bond_accounts_store;

    **bond_accounts_store = BondAccountsStore::init(0, 1, bump);

    Ok(())
}