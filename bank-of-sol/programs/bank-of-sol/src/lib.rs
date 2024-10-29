#![allow(dead_code)]

use anchor_lang::prelude::*;

mod instructions;
mod state;
mod utils;
mod errors;
mod types;

declare_id!("5Xi2uL4MB1RDnDPFHEiec6beWxFqyzFhgYpd2ACjr17m");

#[program]
pub mod bank_of_sol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
