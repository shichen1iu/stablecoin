use crate::constant::*;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump
    )]
    pub config_account: Account<'info, Config>,
    #[account(
        init,
        payer = authority,
        seeds = [SEED_MINT_ACCOUNT],
        bump,
        mint::decimals = MINT_DECIMALS,
        mint::authority = config_account,
        mint::freeze_authority = config_account,
        mint::token_program = token_program,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
    *ctx.accounts.config_account = Config {
        authority: ctx.accounts.authority.key(),
        mint: ctx.accounts.mint.key(),
        liquidation_threshold: LIQUIDATION_THRESHOLD,
        liquidation_bonus: LIQUIDATION_BONUS,
        min_health_factor: MIN_HEALTH_FACTOR,
        bump: ctx.bumps.config_account,
        bump_mint: ctx.bumps.mint,
    };
    Ok(())
}
