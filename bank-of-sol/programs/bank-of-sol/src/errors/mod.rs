use anchor_lang::prelude::*;
use num_enum::TryFromPrimitive;

#[error_code]
#[derive(PartialEq, Eq, TryFromPrimitive)]
pub enum ErrorCode {
    #[msg("Integer overflow")]
    IntegerOverflow,

    #[msg("Conversion failure")]
    ConversionFailure,

    #[msg("Mathematical operation with overflow")]
    MathOverflow,

    #[msg("Out of range integral conversion attempted")]
    OutOfRangeIntegralConversion,

    #[msg("Unexpected account in instruction")]
    UnexpectedAccount,

    #[msg("Unable to deserialize account")]
    UnableToDeserializeAccount,

    #[msg("Invalid account discriminator")]
    InvalidAccountDiscriminator,

    #[msg("Invalid raydium pool")]
    InvalidRaydiumPool,

    #[msg("Epoch not ready to be advanced")]
    EpochNotAdvanceable,

    #[msg("Token not NFT")]
    TokenNotNFT,

    #[msg("Token account empty")]
    TokenAccountEmpty,

    #[msg("Collection not verified")]
    CollectionNotVerified,

    #[msg("Invalid collection")]
    InvalidCollection,

    #[msg("Staking inactive")]
    StakingInactive,

    #[msg("Invalid nft bump")]
    NftBumpError,

    #[msg("Bond account from invalid epoch")]
    BondAccountInvalidEpoch,

    #[msg("Invalid seed index")]
    InvalidSeedIndex,

    #[msg("Negative period value")]
    NegativePeriodValue,

    #[msg("Ineligible for reward")]
    IneligibleForReward,

    #[msg("Invalid account status")]
    InvalidAccountStatus,
}

pub type ProgramResult<T = ()> = std::result::Result<T, ErrorCode>;