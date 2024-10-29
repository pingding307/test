use anchor_lang::prelude::*;

/// A PDA for storing the index of stake accounts
#[account]
pub struct StakeRecordIndex {
    /// The staking index; number of staking PDAs/NFTs staked (8)
    pub index: u64,
    /// The offset; number of stake accounts closed (8)
    pub offset: u64,
    /// The bump of this PDA (1)
    pub bump: u8,
}

impl StakeRecordIndex {
    pub const LEN: usize = 8 + 8 + 8 + 1;

    pub fn init(bump: u8) -> Self {
        Self {
            index: 0,
            offset: 0,
            bump,
        }
    }

    pub fn get_index_of_accounts(&self) -> u64 {
        self.index + self.offset
    }

    pub fn get_offset(&self) -> u64 {
        self.offset
    }

    pub fn pda_closed(&mut self) {
        self.index -= 1;
        self.offset += 1;
    }
}

#[account]
pub struct StakeRecord {
    /// The owner/staker of the NFT (32)
    pub staker: Pubkey,
    /// The mint of the staked NFT (32)
    pub nft_mint: Pubkey,
    /// The staking epoch (8)
    pub staked_epoch: u64,
    /// The staking timestamp (8)
    pub staked_at: i64,
    /// The bump of this PDA (1)
    pub bump: u8,
}

impl StakeRecord {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 1;

    pub fn init(staker: Pubkey, nft_mint: Pubkey, staked_epoch: u64, bump: u8) -> Self {
        let clock = Clock::get().unwrap();
        let staked_at = clock.unix_timestamp;

        Self {
            staker,
            nft_mint,
            staked_epoch,
            staked_at,
            bump,
        }
    }
}