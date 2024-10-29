use anchor_lang::prelude::*;
use anchor_lang::prelude::borsh::{BorshSerialize, BorshDeserialize};
use crate::errors::ErrorCode;

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct Epoch {
    /// The index of the current epoch
    pub index: u64,
    /// The unix timestamp this epoch started at
    pub timestamp: i64,
    /// The slot this epoch started at
    pub slot: u64,
    /// Was the price of Units (U) above peg at the beginning of this epoch
    pub above_peg: bool,
    /// Base interest rate for this epoch
    pub base_rate: u64,
}

pub const HOUR: i64 = 60 * 60;

impl Epoch {
    pub fn init() -> Self {
        let clock = Clock::get().unwrap();

        Self {
            index: 0,
            timestamp: clock.unix_timestamp,
            slot: clock.slot,
            above_peg: true,
            base_rate: 0,
        }
    }

    pub fn advance(&self, above_peg: bool, base_rate: u64) -> Result<Self> {
        let clock = Clock::get().unwrap();

        if clock.unix_timestamp < self.timestamp + HOUR {
            return err!(ErrorCode::EpochNotAdvanceable);
        }

        Ok(Self {
            index: self.index + 1,
            timestamp: clock.unix_timestamp,
            slot: clock.slot,
            above_peg,
            base_rate,
        })
    }
}