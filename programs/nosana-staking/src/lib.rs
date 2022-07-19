mod error;
mod ids;
mod instructions;
mod state;
mod utils;

use error::*;
use ids::*;
use instructions::*;
pub use state::*; // expose stake for cpi

use anchor_lang::prelude::*;
use solana_security_txt::security_txt;

security_txt! {
    name: "Nosana Staking",
    project_url: "http://nosana.io",
    contacts: "email:team@nosana.io,link:https://nosana.io/security,discord:nosana#security",
    policy: "https://github.com/solana-labs/solana/blob/master/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/nosana-ci/nosana-programs",
    auditors: "TBD"
}

#[program]
pub mod nosana_staking {
    use super::*;

    pub fn init_vault(ctx: Context<InitVault>, _bump: u8) -> Result<()> {
        init_vault::handler(ctx)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64, duration: u128) -> Result<()> {
        stake::handler(ctx, amount, duration)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        unstake::handler(ctx)
    }

    pub fn restake(ctx: Context<Restake>) -> Result<()> {
        restake::handler(ctx)
    }

    pub fn topup(ctx: Context<Topup>, amount: u64) -> Result<()> {
        topup::handler(ctx, amount)
    }

    pub fn extend(ctx: Context<Topup>, duration: u128) -> Result<()> {
        extend::handler(ctx, duration)
    }

    pub fn claim(ctx: Context<Claim>, bump: u8) -> Result<()> {
        claim::handler(ctx, bump)
    }
}
