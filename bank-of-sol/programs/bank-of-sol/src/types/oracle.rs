use anchor_lang::prelude::*;
use anchor_lang::prelude::borsh::{BorshSerialize, BorshDeserialize};
use raydium_amm_v3::states::PoolState;
use std::ops::Div;
use crate::utils::account_deserialize;
use crate::utils::math::sqrt_price_to_price;
use crate::types::price::DatedPrice;
use crate::errors::ProgramResult;

/// Gives the price of the given token pair in the given pool
pub fn get_price(a_to_b: bool, pool: &AccountInfo, clock: &Clock) -> ProgramResult<DatedPrice> {
    // Load main account
    let pool_data: PoolState = account_deserialize(pool)?;

    // Compute price
    let price = sqrt_price_to_price(
        a_to_b,
        pool_data.sqrt_price_x64,
        pool_data.mint_decimals_0,
        pool_data.mint_decimals_1,
    )?;

    // Return Price
    Ok(DatedPrice {
        price,
        last_updated_slot: clock.slot,
        unix_timestamp: clock.unix_timestamp as u64,
        ..Default::default()
    })
}

/// Stores price captures
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct PriceStorage {
    pub observations: [DatedPrice; 5],
}

impl Default for PriceStorage {
    fn default() -> Self {
        Self {
            observations: [DatedPrice::default(); 5]
        }
    }
}

/// Time weighted average price
#[derive(Debug)]
pub struct Twap {
    pub twap: u64,
    pub exp: u64,
}

pub fn calculate_twap(observations: [DatedPrice; 5], exp: u64) -> Twap {
    let twap: u64 = observations.map(|x| x.price.value).iter().sum();

    Twap {
        twap: twap.div(10),
        exp,
    }
}

pub fn update_observations(clock: &Clock, a_to_b: bool, raydium_ammv3_pool: &AccountInfo, observations: [DatedPrice; 5]) -> Result<Vec<DatedPrice>> {
    let update = get_price(a_to_b, raydium_ammv3_pool, clock)?;
    let mut observations = Vec::from(observations);
    observations.push(update);

    Ok(observations)
}