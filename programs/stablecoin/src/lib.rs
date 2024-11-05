use anchor_lang::prelude::*;
mod constant;
mod error;
mod instructions;
mod state;

use instructions::*;
declare_id!("4pWg9ug5puVoVwyRciSXSt3Ze1XkeE5idNkpYn5wVNQE");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }

    pub fn deposit_collateral_and_mint_tokens(
        ctx: Context<DepositCollateralAndMintTokens>,
        amount_to_collateral: u64,
        amount_to_mint: u64,
    ) -> Result<()> {
        process_deposit_collateral_and_mint_tokens(ctx, amount_to_collateral, amount_to_mint)
    }
}
