use anchor_lang::prelude::*;
use crate::state::nft_stake_record::StakeRecordIndex;

#[derive(Accounts)]
pub struct CreateStakeRecordIndex<'info> {
    #[account(
        init,
        payer = signer,
        space = StakeRecordIndex::LEN,
        seeds = [
            b"stake-record-index",
            signer.key().as_ref()
        ],
        bump
    )]
    pub stake_record_index: Account<'info, StakeRecordIndex>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_stake_record_index_handler(ctx: Context<CreateStakeRecordIndex>) -> Result<()> {
    let stake_record_index = &mut ctx.accounts.stake_record_index;

    **stake_record_index = StakeRecordIndex::init(ctx.bumps.stake_record_index);

    Ok(())
}