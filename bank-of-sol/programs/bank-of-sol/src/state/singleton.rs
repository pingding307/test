use std::ops::{Add, Sub};

use anchor_lang::prelude::*;
use muldiv::*;
use crate::types::{epoch::Epoch, oracle::PriceStorage, bond::BondStorage, nft_staking::NFTStakingStorage, custody::CustodyAccounts, boardroom::BoardroomBalances};

#[account]
pub struct Singleton {
    /// Authority of the Bank program, can be a multi-sig (32)
    pub authority: Pubkey,
    /// The Raydium AMMv3 pool used by the oracle (32)
    pub pool: Pubkey,
    /// The current epoch (40)
    pub epoch: Epoch,
    /// The last 5 price observations (200)
    pub price_observations: PriceStorage,
    /// Bond info (40)
    pub bonds: BondStorage,
    /// NFT Staking info (80)
    pub nft_staking: NFTStakingStorage,
    /// Unit token mint (32)
    pub unit_mint: Pubkey,
    /// Unit token authority (32)
    pub token_authority: Pubkey,
    /// The token custody accounts (64)
    pub custody: CustodyAccounts,
    /// Total deposits into boardroom
    pub boardroom_deposits: BoardroomBalances,
    /// The bump of `token_authority` (1)
    pub unit_auth_bump: u8,
    /// The bump of the Singleton PDA (1)
    pub bump: u8,
}

impl Singleton {
    pub const LEN: usize = 8 + 32 + 32 + 40 + 200 + 40 + 80 + 32 + 32 + 64 + 1 + 1;

    const INTEREST_RATE_PRECISION: u64 = 1_000_000;
    const ONE_HUNDRED_PCT: u64 = 100 * Self::INTEREST_RATE_PRECISION;

    pub fn init(
        authority: Pubkey, 
        pool: Pubkey, 
        epoch: Epoch, 
        unit_mint: Pubkey, 
        token_authority: Pubkey, 
        unit_custody: Pubkey,
        unit_usdc_lp_custody: Pubkey,
        unit_auth_bump: u8, 
        bump: u8
    ) -> Self {
        Self {
            authority, 
            pool, 
            epoch,
            price_observations: PriceStorage::default(),
            bonds: BondStorage::default(),
            nft_staking: NFTStakingStorage::default(),
            unit_mint,
            token_authority,
            custody: CustodyAccounts::init(unit_custody, unit_usdc_lp_custody),
            boardroom_deposits: BoardroomBalances::default(),
            unit_auth_bump,
            bump,
        }
    }

    pub fn advance(&mut self) {
        self.epoch.advance(true, 10).unwrap();
    }

    pub fn interest_rate(&self) -> u64 {
        self.epoch.base_rate
    }

    pub fn epoch(&self) -> u64 {
        self.epoch.index
    }

    pub fn units_to_bonds(&self, units: u64) -> u64 {
        let bonds = units.mul_div_floor(
            self.interest_rate().add(Self::ONE_HUNDRED_PCT), 
            Self::ONE_HUNDRED_PCT
        ).unwrap();

        bonds
    }

    pub fn bonds_purchased(&mut self, amount: u64) {
        self.bonds.bonds_purchased += amount;
        self.bonds.total_bonds_purchased += amount;
        self.bonds.available_bonds = self.bonds.available_bonds.sub(amount);
    }

    pub fn increment_total_balance_of_staged(&mut self, amount: u64) {
        self.boardroom_deposits.total_deposited_units += amount;
    }

    pub fn decrement_total_balance_of_staged(&mut self, amount: u64) {
        self.boardroom_deposits.total_deposited_units -= amount;
    }
}