use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface};

use crate::constant::SEED_MINT_ACCOUNT;

pub fn mint_tokens<'info>(
    mint: &InterfaceAccount<'info, Mint>,
    token_program: &Interface<'info, TokenInterface>,
    token_account: &InterfaceAccount<'info, TokenAccount>,
    bump: u8,
    amount: u64,
) -> Result<()> {
    let mint_to_ctx = MintTo {
        mint: mint.to_account_info(),
        to: token_account.to_account_info(),
        authority: mint.to_account_info(),
    };
    let signer_seeds: &[&[&[u8]]] = &[&[SEED_MINT_ACCOUNT, &[bump]]];
    let mint_to_ctx_cpi =
        CpiContext::new_with_signer(token_program.to_account_info(), mint_to_ctx, signer_seeds);

    mint_to(mint_to_ctx_cpi, amount)
}

pub fn deposit_sol<'info>(
    from: &Signer<'info>,
    to: &SystemAccount<'info>,
    system_program: &Program<'info, System>,
    amount: u64,
) -> Result<()> {
    let transfer_ctx = Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
    };
    let transfer_ctx_cpi = CpiContext::new(system_program.to_account_info(), transfer_ctx);
    transfer(transfer_ctx_cpi, amount)
}
