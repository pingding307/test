use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, MintTo, mint_to},
    associated_token::AssociatedToken,
};

use crate::state::{singleton::Singleton, nft_stake_record::{StakeRecord, StakeRecordIndex}};
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(_seed_index: u64)]
pub struct ClaimStakingReward<'info> {
    #[account(
        seeds = [b"singleton"],
        bump = singleton.bump,
        has_one = unit_mint
    )]
    pub singleton: Account<'info, Singleton>,

    #[account(
        mut,
        seeds = [
            b"stake-record",
            signer.key().as_ref(),
            _seed_index.to_be_bytes().as_ref(),
        ],
        bump = stake_record.bump,
        constraint = _seed_index <= stake_record_index.get_index_of_accounts() @ ErrorCode::InvalidSeedIndex,
        constraint = _seed_index > stake_record_index.get_offset() @ ErrorCode::InvalidSeedIndex
    )]
    pub stake_record: Account<'info, StakeRecord>,

    #[account(
        seeds = [
            b"stake-record-index",
            signer.key().as_ref()
        ],
        bump = stake_record_index.bump
    )]
    pub stake_record_index: Account<'info, StakeRecordIndex>,

    #[account(
        mut,
        mint::authority = token_authority
    )]
    pub unit_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = unit_mint,
        associated_token::authority = signer
    )]
    pub unit_receive_account: Account<'info, TokenAccount>,

    /// CHECK: This account is not read or written
    #[account(
        seeds = [
            b"token-authority"
        ],
        bump
    )]
    pub token_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimStakingReward<'info> {
    pub fn mint_token_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.unit_mint.to_account_info(),
            to: self.unit_receive_account.to_account_info(),
            authority: self.token_authority.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn claim_reward_handler(ctx: Context<ClaimStakingReward>, _seed_index: u64) -> Result<()> {
    let singleton = &ctx.accounts.singleton;

    let staked_at = ctx.accounts.stake_record.staked_at;
    let minimum_stake_period = singleton.nft_staking.minimum_period;
    let staking_status = singleton.nft_staking.status;
    let unit_auth_bump = singleton.unit_auth_bump;

    require_eq!(staking_status, true, ErrorCode::StakingInactive);

    let authority_seed = &[&b"token-authority"[..], &[unit_auth_bump]];
    // TODO: reward eligibility
    let (eligible_for_reward, reward_units, time) = (true, (50e6) as u64, Clock::get().unwrap().unix_timestamp);

    if eligible_for_reward {
        mint_to(
            ctx.accounts.mint_token_ctx().with_signer(&[&authority_seed[..]]),
            reward_units
        )?;
    } else {
        return err!(ErrorCode::IneligibleForReward);
    }

    ctx.accounts.stake_record.staked_at = time;

    Ok(())
}