use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, MintTo, Transfer, CloseAccount, mint_to, transfer, close_account},
    associated_token::AssociatedToken,
};

use crate::state::{nft_stake_record::{StakeRecord, StakeRecordIndex}, singleton::Singleton};
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(_seed_index: u64)]
pub struct UnstakeNft<'info> {
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
            staker.key().as_ref(),
            _seed_index.to_be_bytes().as_ref()
        ],
        bump = stake_record.bump,
        has_one = nft_mint,
        has_one = staker,
        close = staker,
        constraint = _seed_index <= stake_record_index.get_index_of_accounts() @ ErrorCode::InvalidSeedIndex,
        constraint = _seed_index > stake_record_index.get_offset() @ ErrorCode::InvalidSeedIndex
    )]
    pub stake_record: Account<'info, StakeRecord>,

    #[account(
        mut,
        seeds = [
            b"stake-record-index",
            staker.key().as_ref()
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
        payer = staker,
        associated_token::mint = unit_mint,
        associated_token::authority = staker
    )]
    pub unit_receive_account: Account<'info, TokenAccount>,

    #[account(
        mint::decimals = 0,
        constraint = nft_mint.supply == 1 @ ErrorCode::TokenNotNFT
    )]
    nft_mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = staker,
        associated_token::mint = nft_mint,
        associated_token::authority = staker
    )]
    nft_receive_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = nft_authority,
        constraint = nft_custody.amount == 1 @ ErrorCode::TokenAccountEmpty
    )]
    pub nft_custody: Box<Account<'info, TokenAccount>>,

    /// CHECK: This account is not read or written
    #[account(
        seeds = [
            b"token-authority"
        ],
        bump = singleton.unit_auth_bump
    )]
    pub token_authority: UncheckedAccount<'info>,

    /// CHECK: This account is not read or written
    #[account(
        seeds = [
            b"nft-authority"
        ],
        bump = singleton.nft_staking.nft_auth_bump
    )]
    pub nft_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub staker: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> UnstakeNft<'info> {
    pub fn mint_token_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.unit_mint.to_account_info(),
            to: self.unit_receive_account.to_account_info(),
            authority: self.token_authority.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn transfer_nft_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.nft_custody.to_account_info(),
            to: self.nft_receive_account.to_account_info(),
            authority: self.nft_authority.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn close_account_ctx(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.nft_custody.to_account_info(),
            destination: self.staker.to_account_info(),
            authority: self.nft_authority.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn unstake_nft_handler(ctx: Context<UnstakeNft>, _seed_index: u64) -> Result<()> {
    let singleton = &ctx.accounts.singleton;
    let stake_record_index = &mut ctx.accounts.stake_record_index;

    stake_record_index.pda_closed();

    let staked_at = ctx.accounts.stake_record.staked_at;
    let minimum_stake_period = singleton.nft_staking.minimum_period;
    let staking_active = singleton.nft_staking.status;
    let unit_auth_bump = singleton.unit_auth_bump;
    let nft_auth_bump = singleton.nft_staking.nft_auth_bump;
    
    // TODO: calculate this properly
    let (eligible_for_reward, reward_units, time) = (true, (50e6) as u64, Clock::get().unwrap().unix_timestamp);

    let unit_auth_seed = &[&b"token-authority"[..], &[unit_auth_bump]];
    let nft_auth_seed = &[&b"nft-authority"[..], &[nft_auth_bump]];

    if eligible_for_reward && staking_active {
        // mint units
        mint_to(
            ctx.accounts.mint_token_ctx().with_signer(&[&unit_auth_seed[..]]),
            reward_units
        )?;
    }

    // Transfer nft
    transfer(
        ctx.accounts.transfer_nft_ctx().with_signer(&[&nft_auth_seed[..]]),
        1
    )?;

    // Close nft custody account
    close_account(ctx.accounts.close_account_ctx().with_signer(&[&nft_auth_seed[..]]))?;

    Ok(())
}