#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;
pub mod errors;

// pub use errors::ErrorCodes;
 use instructions::*;
pub use states::*;

declare_id!("A1QHrhtGGSMDGaAtXvHwCGLFcABbjVnfuZikLkQipGDn");

#[program]
pub mod whitelist_token_sale {
    use super::*;

    /**
     * Initialize a whitelist with the given name, associated with the authority of this transaction.
     */
    pub fn init_whitelist(
        ctx: Context<InitWhitelist>,
        name: String
    ) -> Result<()> {
        instructions::init_whitelist(ctx, name)
    }

    // Token sales
    pub fn init_sale
    (ctx: Context<InitSale>, 
      token_price: u64, 
      max_per_wallet: u64
    ) -> Result<()> {
        instructions::init_sale(ctx, token_price, max_per_wallet)
    }

    // pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
    //     instructions::buy_tokens(ctx, amount)
    // }
    pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
      instructions::buy_tokens(ctx, amount)
  }

    /**
     * Delete a whitelist create with `init_whitelist` with the given name and bump,
     * associated with the authority of this transaction. The remaining lamports are sent to the
     * authority of this transaction.
     */
    pub fn delete_whitelist(
        ctx: Context<DeleteWhitelist>,
        name: String,
    ) -> Result<()> {
        instructions::delete_whitelist(ctx, name)
    }

    /**
     * Check if the given whitelist exists (has been initalized).
     */
    pub fn check_whitelist(
        ctx: Context<CheckWhitelist>,
        owner: Pubkey,
        name: String,
    ) -> Result<()> {
        instructions::check_whitelist(ctx, owner, name)
    }
    
    /**
     * Add the account `account_to_add` with the specified bump to the `whitelist` account.
     */
    pub fn add_to_whitelist(
        ctx: Context<AddToWhitelist>,
        account_to_add: Pubkey
    ) -> Result<()> {
        instructions::add_to_whitelist(ctx, account_to_add)
    }

    /**
     * Remove the account `account_to_delete` with the specified bump from the `whitelist` account.
     * Remaining lamports are sent to the authority of this transaction.
     */
    pub fn remove_from_whitelist(
        ctx: Context<RemoveFromWhitelist>,
        account_to_delete: Pubkey,
    ) -> Result<()> {
        instructions::remove_from_whitelist(ctx, account_to_delete)
    }

    /**
     * Check if the account `account_to_check` is whitelisted in the given `whitelist` account.
     */
    pub fn check_whitelisted(
        ctx: Context<CheckWhitelisted>,
        account_to_check: Pubkey,
    ) -> Result<()> {
        instructions::check_whitelisted(ctx, account_to_check)
    }
}

