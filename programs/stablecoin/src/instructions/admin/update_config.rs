use crate::constant::*;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]
    pub config_account: Account<'info, Config>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

pub fn process_update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
    ctx.accounts.config_account.min_health_factor = min_health_factor;
    Ok(())
}
