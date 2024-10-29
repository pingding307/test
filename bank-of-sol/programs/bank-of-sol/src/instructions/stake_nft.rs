use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, transfer, TokenAccount, Transfer},
    metadata::{MasterEditionAccount, MetadataAccount, Metadata},
    associated_token::AssociatedToken
};

use crate::state::{singleton::Singleton, nft_stake_record::{StakeRecord, StakeRecordIndex}};
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        seeds = [b"singleton"],
        bump = singleton.bump
    )]
    pub singleton: Account<'info, Singleton>,

    #[account(
        seeds = [
            b"stake-record-index",
            signer.key().as_ref(),
        ],
        bump = stake_record_index.bump
    )]
    pub stake_record_index: Account<'info, StakeRecordIndex>,

    #[account(
        init,
        payer = signer,
        space = StakeRecord::LEN,
        seeds = [
            b"stake-record",
            signer.key().as_ref(),
            (stake_record_index.index + stake_record_index.offset).to_be_bytes().as_ref()
        ],
        bump
    )]
    pub stake_record: Account<'info, StakeRecord>,

    #[account(
        mint::decimals = 0,
        constraint = nft_mint.supply == 1 @ ErrorCode::TokenNotNFT
    )]
    nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = signer,
        constraint = nft_token.amount == 1 @ ErrorCode::TokenAccountEmpty
    )]
    nft_token: Account<'info, TokenAccount>,

    #[account(
        seeds = [
            b"metadata",
            Metadata::id().as_ref(),
            nft_mint.key().as_ref()
        ],
        seeds::program = Metadata::id(),
        bump,
        constraint = nft_metadata.collection.as_ref().unwrap().verified @ ErrorCode::CollectionNotVerified,
        constraint = nft_metadata.collection.as_ref().unwrap().key == singleton.nft_staking.collection @ ErrorCode::InvalidCollection
    )]
    nft_metadata: Box<Account<'info, MetadataAccount>>,

    #[account(
        seeds = [
            b"metadata",
            Metadata::id().as_ref(),
            nft_mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = Metadata::id(),
        bump
    )]
    nft_edition: Box<Account<'info, MasterEditionAccount>>,

    /// CHECK: This account is not read or written
    #[account(
        seeds = [
            b"nft-authority",
            singleton.key().as_ref()
        ],
        bump = singleton.nft_staking.nft_auth_bump
    )]
    pub nft_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = nft_mint,
        associated_token::authority = nft_authority
    )]
    pub nft_custody: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>, 
}

impl<'info> Stake<'info> {
    pub fn transfer_nft_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.nft_token.to_account_info(),
            to: self.nft_custody.to_account_info(),
            authority: self.signer.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn stake_handler(ctx: Context<Stake>) -> Result<()> {
    let staking_status = ctx.accounts.singleton.nft_staking.status;

    require_eq!(staking_status, true, ErrorCode::StakingInactive);

    let staker = ctx.accounts.signer.key();
    let nft_mint = ctx.accounts.nft_mint.key();
    let staked_epoch = ctx.accounts.singleton.epoch.index;
    let bump = ctx.bumps.stake_record;

    transfer(ctx.accounts.transfer_nft_ctx(), 1)?;

    let nft_record = &mut ctx.accounts.stake_record;
    **nft_record = StakeRecord::init(staker, nft_mint, staked_epoch, bump);

    Ok(())
}