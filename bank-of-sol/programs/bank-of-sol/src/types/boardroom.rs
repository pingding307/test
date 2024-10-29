use anchor_lang::prelude::*;
use anchor_lang::prelude::borsh::{BorshSerialize, BorshDeserialize};

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub enum BoardroomStatus {
    Fluid { fluid_until: u64 },
    Frozen { became_frozen: u64 },
    Locked { locked_until: u64 },
}

impl BoardroomStatus {
    pub fn frozen(epoch: u64) -> Self {
        Self::Frozen { became_frozen: epoch }
    }

    pub fn fluid(epoch: u64) -> Self {
        Self::Fluid { fluid_until: epoch }
    }

    pub fn locked(epoch: u64) -> Self {
        Self::Locked { locked_until: epoch }
    }
}

#[derive(Debug, Default, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct BoardroomBalances {
    pub total_deposited_units: u64,
}