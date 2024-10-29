pub mod math;

use anchor_lang::{
    prelude::{msg, AccountDeserialize, AccountInfo},
    Discriminator, Key,
};

use std::convert::AsMut;

use crate::errors::{ErrorCode, ProgramResult};

pub fn account_deserialize<T: AccountDeserialize + Discriminator>(
    account: &AccountInfo<'_>,
) -> ProgramResult<T> {
    let data = account.clone().data.borrow().to_owned();
    let discriminator = data.get(..8).ok_or_else(|| {
        msg!(
            "Account {:?} does not have enough bytes to be deserialized",
            account.key()
        );
        ErrorCode::UnableToDeserializeAccount
    })?;
    if discriminator != T::discriminator() {
        msg!(
            "Expected discriminator for account {:?} ({:?}) is different from received {:?}",
            account.key(),
            T::discriminator(),
            discriminator
        );
        return Err(ErrorCode::InvalidAccountDiscriminator);
    }

    let mut data: &[u8] = &data;
    let user: T = T::try_deserialize(&mut data).map_err(|_| {
        msg!("Account {:?} deserialization failed", account.key());
        ErrorCode::UnableToDeserializeAccount
    })?;

    Ok(user)
}

pub fn copy_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Copy,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
    a
}