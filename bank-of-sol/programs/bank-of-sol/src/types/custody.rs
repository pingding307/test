use anchor_lang::prelude::{*, borsh::{BorshSerialize, BorshDeserialize}};

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
pub struct CustodyAccounts {
    pub unit_custody: Pubkey,
    pub unit_usdc_lp_custody: Pubkey,
}

impl CustodyAccounts {
    pub fn init(unit_custody: Pubkey, unit_usdc_lp_custody: Pubkey) -> Self {
        Self { unit_custody, unit_usdc_lp_custody }
    }
}