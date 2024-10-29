use anchor_lang::prelude::*;

use crate::types::bond::UserBondStorage;

/**
 * Bond PDA architecture
 * 
 * Each time a user purchases bonds, a new PDA will be created to store information on the bond purchase:
 *  - The amount of bonds
 *  - The interest rate
 *  - The epoch purchased in
 * 
 * The `number_of_accounts` field is incremented by 1 each time a new PDA is created, 
 * this is to make it easier to keep track of all the bond PDAs.
 * 
 * When a user completely redeems the bonds from one of the bond accounts, the PDA is closed,
 * an offset number is incremented to keep track of which account is now the oldest.
 * 
 */

/// A PDA for storing the number of bond accounts for a particular user
#[account]
pub struct BondAccountsStore {
    /// The number of bond account PDAs (8)
    pub number_of_accounts: u64,
    /// The offset (8)
    pub offset: u64,
    /// The bump of this PDA (1)
    pub bump: u8,
}

impl BondAccountsStore {
    pub const LEN: usize = 8 + 8 + 8 + 1;

    pub fn init(number_of_accounts: u64, offset: u64, bump: u8) -> Self {
        Self {
            number_of_accounts,
            offset,
            bump,
        }
    }

    pub fn increment(&mut self) {
        self.number_of_accounts += 1;
    }
    // TODO: this is probably very broken
    pub fn pda_closed(&mut self) {
        self.number_of_accounts -= 1;
        self.offset += 1;
    }

    pub fn get_index_of_accounts(&self) -> u64 {
        self.offset + self.number_of_accounts
    }

    pub fn get_offset(&self) -> u64 {
        self.offset
    }
}

#[account]
pub struct BondAccount {
    /// The user who owns this PDA (32)
    pub user: Pubkey,
    /// Bond storage (24)
    pub bonds: UserBondStorage,
    /// The index of these bonds; the total amount of bonds purchased over all time (8)
    pub index: u64,
    /// The bump of this PDA (1)
    pub bump: u8,
}

impl BondAccount {
    pub const LEN: usize = 8 + 32 + 24 + 1;

    pub fn init(user: Pubkey, bump: u8, amount: u64, epoch: u64, interest_rate: u64, index: u64) -> Self {
        Self {
            user,
            bonds: UserBondStorage::new(amount, epoch, interest_rate),
            bump,
            index,
        }
    }
}