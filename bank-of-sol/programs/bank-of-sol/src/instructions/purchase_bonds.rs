use anchor_lang::prelude::*;

use crate::state::{bond_account::{BondAccount, BondAccountsStore}, singleton::Singleton};
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(_seed_index: u64)]
pub struct PurchaseBonds<'info> {
    #[account(
        mut,
        seeds = [
            b"bond-account",
            signer.key().as_ref(),
            _seed_index.to_be_bytes().as_ref()
        ],
        bump = bond_account.bump,
        constraint = bond_account.bonds.epoch == 0 || bond_account.bonds.epoch == singleton.epoch() @ ErrorCode::BondAccountInvalidEpoch,
        constraint = _seed_index <= bond_accounts_store.get_index_of_accounts() @ ErrorCode::InvalidSeedIndex,
        constraint = _seed_index > bond_accounts_store.get_offset() @ ErrorCode::InvalidSeedIndex
    )]
    pub bond_account: Account<'info, BondAccount>,

    #[account(
        mut,
        seeds = [
            b"bond-accounts-store",
            signer.key().as_ref()
        ],
        bump = bond_accounts_store.bump,
    )]
    pub bond_accounts_store: Account<'info, BondAccountsStore>,

    #[account(
        mut,
        seeds = [
            b"singleton"
        ],
        bump = singleton.bump
    )]
    pub singleton: Account<'info, Singleton>,

    pub signer: Signer<'info>,
}

pub fn purchase_bonds_handler(ctx: Context<PurchaseBonds>, _seed_index: u64, units: u64) -> Result<()> {
    let bond_account = &mut ctx.accounts.bond_account;
    let singleton = &mut ctx.accounts.singleton;

    let amount = singleton.units_to_bonds(units);
    let rate = singleton.interest_rate();
    let epoch = singleton.epoch();

    if bond_account.bonds.amount == 0 {
        bond_account.index = singleton.bonds.bonds_purchased;
    }

    bond_account.bonds.epoch = epoch;
    bond_account.bonds.interest_rate = rate;
    bond_account.bonds.amount += amount;

    singleton.bonds_purchased(amount);

    Ok(())
}