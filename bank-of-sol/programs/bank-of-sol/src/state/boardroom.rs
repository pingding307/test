use anchor_lang::prelude::*;

use crate::types::boardroom::BoardroomStatus;

/// A PDA for storing user boardroom state
#[account]
pub struct BoardroomAccount {
    /// The amount of shares held by this user (8)
    pub shares: u64,
    /// The amount of futures held by this user (8)
    pub futures: u64,
    /// The timestamp when the user last deposited an asset into the Boardroom (8)
    pub last_deposited_timestamp: i64,
    /// The epoch when the user last deposited (8)
    pub epoch_last_deposited: u64,
    /// The status of this account (16)
    pub status: BoardroomStatus,
    /// Staged balances waiting to be staked (16)
    pub staged_balance: u64,
    /// The bump of this PDA (1)
    pub bump: u8,
}

impl BoardroomAccount {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 8 + 16 + 16 + 1;

    pub fn init(epoch: u64, bump: u8) -> Self {
        let clock = Clock::get().unwrap();

        Self {
            shares: 0,
            futures: 0,
            last_deposited_timestamp: clock.unix_timestamp,
            epoch_last_deposited: epoch,
            status: BoardroomStatus::frozen(epoch),
            staged_balance: 0,
            bump,
        }
    }

    pub fn increment_balance_of_staged(&mut self, amount: u64) {
        self.staged_balance += amount;
    }

    pub fn decrement_balance_of_staged(&mut self, amount: u64) {
        self.staged_balance -= amount;
    }

    pub fn only_frozen_or_locked(&self) -> bool {
        match self.status {
            BoardroomStatus::Frozen { .. } => true,
            BoardroomStatus::Locked { .. } => true,
            _ => false,
        }
    }
}