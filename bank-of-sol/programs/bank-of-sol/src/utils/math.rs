use decimal_wad::decimal::U192;
use decimal_wad::rate::U128;
use raydium_amm_v3::libraries::U256;
use crate::errors::{ErrorCode, ProgramResult};
use crate::types::price::Price;

/// Transform sqrt price to normal price scaled by 2^64
fn sqrt_price_to_x64_price(sqrt_price: u128, decimals_a: u8, decimals_b: u8) -> U192 {
    let sqrt_price = U256::from(sqrt_price);
    let price = (sqrt_price * sqrt_price) >> U256::from(64);
    let price_u256 = if decimals_a >= decimals_b {
        price * U256::from(ten_pow(decimals_a - decimals_b))
    } else {
        price / U256::from(ten_pow(decimals_b - decimals_a))
    };
    debug_assert_eq!(price_u256.0[3], 0, "price overflow: {:?}", price_u256); // should not overflow because of the shift
    U192([price_u256.0[0], price_u256.0[1], price_u256.0[2]])
}

pub fn sqrt_price_to_price(
    a_to_b: bool,
    sqrt_price: u128,
    decimals_a: u8,
    decimals_b: u8,
) -> ProgramResult<Price> {
    const MAX_INTEGER_PART: u128 = u64::MAX as u128;

    if sqrt_price == 0 {
        return Ok(Price { value: 0, exp: 0 })
    }

    let x64_price = if a_to_b {
        sqrt_price_to_x64_price(sqrt_price, decimals_a, decimals_b)
    } else {
        // invert the sqrt price
        let inverted_sqrt_price = (U192::one() << 128) / sqrt_price;
        sqrt_price_to_x64_price(inverted_sqrt_price.as_u128(), decimals_b, decimals_a)
    };
    let integer_part_u192 = x64_price >> U192::from(64);
    let integer_part_u128 = integer_part_u192.as_u128();

    let (exp, factor) = match integer_part_u128 {
        0 => (18, 10_u64.pow(18)),
        1..=9 => (17, 10_u64.pow(17)),
        10..=99 => (16, 10_u64.pow(16)),
        100..=999 => (15, 10_u64.pow(15)),
        1000..=9999 => (14, 10_u64.pow(14)),
        10000..=99999 => (13, 10_u64.pow(13)),
        100000..=999999 => (12, 10_u64.pow(12)),
        1000000..=9999999 => (11, 10_u64.pow(11)),
        10000000..=99999999 => (10, 10_u64.pow(10)),
        100000000..=999999999 => (9, 10_u64.pow(9)),
        1000000000..=9999999999 => (8, 10_u64.pow(8)),
        10000000000..=99999999999 => (7, 10_u64.pow(7)),
        100000000000..=999999999999 => (6, 10_u64.pow(6)),
        1000000000000..=9999999999999 => (5, 10_u64.pow(5)),
        10000000000000..=99999999999999 => (4, 10_u64.pow(4)),
        100000000000000..=999999999999999 => (3, 10_u64.pow(3)),
        1000000000000000..=9999999999999999 => (2, 10_u64.pow(2)),
        10000000000000000..=99999999999999999 => (1, 10_u64.pow(1)),
        100000000000000000..=MAX_INTEGER_PART => (0, 1),
        _ => return Err(ErrorCode::OutOfRangeIntegralConversion),
    };
    let value_u192 = (x64_price * U192::from(factor)) >> U192::from(64);
    let value = value_u192.as_u64();

    Ok(Price { value, exp })
}

pub fn u64_div_to_price(numerator: u64, denominator: u64) -> Price {
    // this implementation aims to keep as much precision as possible
    // choose exp to be the nearest power of 10 to the denominator
    // so that the result is in the range [0, 10^18]
    let (exp, ten_pow_exp) = match denominator {
        0 => panic!("Creating a price by dividing by 0"),
        1..=10 => (0, 1_u64),
        11..=100 => (1, 10),
        101..=1000 => (2, 100),
        1001..=10000 => (3, 1000),
        10001..=100000 => (4, 10000),
        100001..=1000000 => (5, 100000),
        1000001..=10000000 => (6, 1000000),
        10000001..=100000000 => (7, 10000000),
        100000001..=1000000000 => (8, 100000000),
        1000000001..=10000000000 => (9, 1000000000),
        10000000001..=100000000000 => (10, 10000000000),
        100000000001..=1000000000000 => (11, 100000000000),
        1000000000001..=10000000000000 => (12, 1000000000000),
        10000000000001..=100000000000000 => (13, 10000000000000),
        100000000000001..=1000000000000000 => (14, 100000000000000),
        1000000000000001..=10000000000000000 => (15, 1000000000000000),
        10000000000000001..=100000000000000000 => (16, 10000000000000000),
        100000000000000001..=1000000000000000000 => (17, 100000000000000000),
        _ => (18, 1000000000000000000),
    };
    let numerator_scaled = U128::from(numerator) * U128::from(ten_pow_exp);
    let price_value = numerator_scaled / U128::from(denominator);
    Price {
        value: price_value.as_u64(),
        exp,
    }
}

pub fn ten_pow(exponent: u8) -> u128 {
    let value: u128 = match exponent {
        30 => 1_000_000_000_000_000_000_000_000_000_000,
        29 => 100_000_000_000_000_000_000_000_000_000,
        28 => 10_000_000_000_000_000_000_000_000_000,
        27 => 1_000_000_000_000_000_000_000_000_000,
        26 => 100_000_000_000_000_000_000_000_000,
        25 => 10_000_000_000_000_000_000_000_000,
        24 => 1_000_000_000_000_000_000_000_000,
        23 => 100_000_000_000_000_000_000_000,
        22 => 10_000_000_000_000_000_000_000,
        21 => 1_000_000_000_000_000_000_000,
        20 => 100_000_000_000_000_000_000,
        19 => 10_000_000_000_000_000_000,
        18 => 1_000_000_000_000_000_000,
        17 => 100_000_000_000_000_000,
        16 => 10_000_000_000_000_000,
        15 => 1_000_000_000_000_000,
        14 => 100_000_000_000_000,
        13 => 10_000_000_000_000,
        12 => 1_000_000_000_000,
        11 => 100_000_000_000,
        10 => 10_000_000_000,
        9 => 1_000_000_000,
        8 => 100_000_000,
        7 => 10_000_000,
        6 => 1_000_000,
        5 => 100_000,
        4 => 10_000,
        3 => 1_000,
        2 => 100,
        1 => 10,
        0 => 1,
        _ => panic!("no support for exponent: {exponent}"),
    };

    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use test_case::test_case;

    proptest! {
        #[test]
        fn test_u64_div_to_price(numerator in 1_u64..=u64::MAX, denominator in 1_u64..=u64::MAX) {
            let price = u64_div_to_price(numerator, denominator);
            let price_f64: f64 =  price.into();
            let expected_price_f64: f64 = numerator as f64 / denominator as f64;
            prop_assert!((price_f64 - expected_price_f64).abs() < expected_price_f64/1000000000.0, "price_f64: {}, expected_price_f64: {}", price_f64, expected_price_f64);
        }
    }

    #[test_case(true, 1.0, 1.0, 6, 6;"one equal decimals a to b")]
    #[test_case(true, 1.0, 1000.0, 9, 6;"one diff decimals a to b")]
    #[test_case(true, 1.0, 0.001, 6, 9;"one diff decimals2 a to b")]
    #[test_case(true, 1000.0, 1000000.0, 6, 6;"1k equal decimals a to b")]
    #[test_case(true, 2.0, 4.0, 6, 6;"two equal decimals a to b")]
    #[test_case(false, 1.0, 1.0, 6, 6;"one equal decimals b to a")]
    #[test_case(false, 0.5, 4.0, 6, 6;"two equal decimals b to a")]
    #[test_case(false, 0.001, 1000000.0, 6, 6;"1/1k equal decimals b to a")]
    fn test_sqrt_price_conversion(
        a_to_b: bool,
        sqrt_price: f64,
        expected_price_f64: f64,
        decimals_a: u8,
        decimals_b: u8,
    ) {
        let sqrt_price = (sqrt_price * (2.0_f64.powi(64))) as u128;
        let price = sqrt_price_to_price(a_to_b, sqrt_price, decimals_a, decimals_b).unwrap();
        let price_f64: f64 = price.into();
        assert!(
            (price_f64 - expected_price_f64).abs() < expected_price_f64 / 10000000.0,
            "price_f64: {}, expected_price_f64: {}",
            price_f64,
            expected_price_f64
        );
    }
}