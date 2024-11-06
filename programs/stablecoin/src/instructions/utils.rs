use crate::constant::*;
use crate::error::*;
use crate::state::*;
use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

pub fn check_health_factor(
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<()> {
    let health_factor = calculate_health_factor(collateral, config, price_feed)?;
    require!(
        health_factor >= config.min_health_factor,
        CustomErrorCode::HealthFactorTooLow
    );
    Ok(())
}

pub fn calculate_health_factor(
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let collateral_value_in_usd = get_usd_value(&collateral.lamports_balance, price_feed)?;
    //collateral_adjusted_for_liquidation_threshold是能够mint的最大金额
    let collateral_adjusted_for_liquidation_threshold =
        collateral_value_in_usd * config.liquidation_threshold / 100;

    if collateral.amount_minted == 0 {
        msg!("Health Factor Max");
        return Ok(u64::MAX);
    }

    let health_factor = collateral_adjusted_for_liquidation_threshold / collateral.amount_minted;
    Ok(health_factor)
}

pub fn get_usd_value(lamports_balance: &u64, price_feed: &Account<PriceUpdateV2>) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;

    require!(price.price > 0, CustomErrorCode::InvalidPrice);

    //price.price的decimals为8,lamports的decimals为9,所以需要乘以10
    let price_in_usd = price.price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;

    let amount_in_usd = (*lamports_balance as u128 * price_in_usd) / LAMPORTS_PER_SOL as u128;
    Ok(amount_in_usd as u64)
}

// Given USD amount, return lamports based on current SOL price
pub fn get_lamports_from_usd(
    amount_in_usd: &u64,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    let feed_id = get_feed_id_from_hex(FEED_ID)?;
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;

    // Check price is positive
    require!(price.price > 0, CustomErrorCode::InvalidPrice);

    // Adjust price to match lamports precision (9 decimals)
    // Example: Assuming 1 SOL = $2.00
    // price.price = 200_000_000 (from Pyth, 8 decimals)
    // price_in_usd = 200_000_000 * 10 = 2_000_000_000 (9 decimals)
    let price_in_usd = price.price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;

    // Calculate lamports
    // Example: Convert $0.50 to lamports when 1 SOL = $2.00
    // amount_in_usd = 500_000_000 (user input, 9 decimals for $0.50)
    // LAMPORTS_PER_SOL = 1_000_000_000
    // price_in_usd = 2_000_000_000 (as calculated above)
    // amount_in_lamports = (500_000_000 * 1_000_000_000) / 2_000_000_000 = 250_000_000 (0.25 SOL)
    let amount_in_lamports = ((*amount_in_usd as u128) * (LAMPORTS_PER_SOL as u128)) / price_in_usd;

    // EXAMPLE LOGS
    // Program log: *** CONVERT SOL TO USD ***
    // Program log: Price in USD (for 1 SOL): 136.194634200
    // Program log: USD Amount: 1.500000000
    // Program log: SOL Value: 0.011013649
    msg!("*** CONVERT SOL TO USD ***");
    msg!("SOL/USD Price : {:.9}", price_in_usd as f64 / 1e9);
    msg!("USD Amount    : {:.9}", *amount_in_usd as f64 / 1e9);
    msg!("SOL Value     : {:.9}", amount_in_lamports as f64 / 1e9);

    Ok(amount_in_lamports as u64)
}
