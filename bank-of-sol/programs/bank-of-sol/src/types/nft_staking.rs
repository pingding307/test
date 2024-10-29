use anchor_lang::prelude::{*, borsh::{BorshSerialize, BorshDeserialize}};

/// Stores info on the NFT staking system
#[derive(Debug, Default, Clone, BorshSerialize, BorshDeserialize)]
pub struct NFTStakingStorage {
    /// The status of the staking (1)
    pub status: bool,
    /// The staking authority (32)
    pub authority: Pubkey,
    /// The verified collection address of the NFT (32)
    pub collection: Pubkey,
    /// The minimum stake period to be eligible for rewards - in epochs (8)
    pub minimum_period: u64,
    /// The bump of this PDA (1)
    pub bump: u8,
    /// The bump of the NFT authority PDA (1)
    pub nft_auth_bump: u8,
}