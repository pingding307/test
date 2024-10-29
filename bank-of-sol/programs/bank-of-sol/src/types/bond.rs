use anchor_lang::prelude::{*, borsh::{BorshSerialize, BorshDeserialize}};

/// Stores bond specific data
#[derive(Debug, Default, Clone, BorshSerialize, BorshDeserialize)]
pub struct BondStorage {
    /// The number of Units the protocol is willing to borrow/how many bonds are available to purchase, reset each epoch
    pub available_bonds: u64,
    /// The number of bonds purchased this epoch
    pub bonds_purchased: u64,
    /// The total number of bonds ever purchased
    pub total_bonds_purchased: u64,
    /// The total number of bonds ever redeemed
    pub redeemed: u64,
    /// The redeemable index; the total number of bonds that have ever been redeemable, include previously redeemed bonds
    pub redeemable_index: u64,
}

/// Stores information on user purchased bonds
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct UserBondStorage {
    /// The amount of bonds purchased in `epoch`
    pub amount: u64,
    /// The epoch number of bond issueance
    pub epoch: u64,
    /// The interest rate at the time of purchase
    pub interest_rate: u64,
}

impl UserBondStorage {
    pub fn new(amount: u64, epoch: u64, interest_rate: u64) -> Self {
        Self { amount, epoch, interest_rate }
    }
}