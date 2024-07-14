// use anchor_lang::prelude::*;
// use crate::{errors::CustomError, states::{Sale, User, WhitelistEntry}};

// pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
//     let sale = &ctx.accounts.sale;
//     let user = &mut ctx.accounts.user;

//     require!(user.purchased_amount + amount <= sale.max_per_wallet, CustomError::MaxPurchaseLimitReached);
    
//     let total_price = amount.checked_mul(sale.token_price).ok_or(ProgramError::InvalidArgument)?;
//     require!(ctx.accounts.buyer.lamports() >= total_price, CustomError::InsufficientFunds);

//     user.purchased_amount += amount;
//     Ok(())
// }

// #[derive(Accounts)]
// pub struct BuyTokens<'info> {
//     #[account(mut)]
//     buyer: Signer<'info>,
//     #[account(
//         mut,
//         seeds=[b"sale", sale.authority.key().as_ref(), sale.whitelist.as_ref()],
//         bump = sale.bump
//     )]
//     sale: Account<'info, Sale>,
//     #[account(
//         seeds=[
//             buyer.key().as_ref(),
//             sale.whitelist.as_ref()
//         ],
//         bump = whitelist_entry.bump
//     )]
//     whitelist_entry: Account<'info, WhitelistEntry>,
//     #[account(
//         init_if_needed,
//         payer=buyer,
//         space=User::SIZE,
//         seeds=[b"user", buyer.key().as_ref(), sale.key().as_ref()],
//         bump
//     )]
//     user: Account<'info, User>,
//     system_program: Program<'info, System>,
// }

// impl<'info> BuyTokens<'info> {
//     pub fn new(
//         buyer: Signer<'info>,
//         sale: Account<'info, Sale>,
//         whitelist_entry: Account<'info, WhitelistEntry>,
//         user: Account<'info, User>,
//         system_program: Program<'info, System>,
//     ) -> Self {
//         Self {
//             buyer,
//             sale,
//             whitelist_entry,
//             user,
//             system_program,
//         }
//     }
// }

use anchor_lang::prelude::*;
use crate::{errors::CustomError, states::{Sale, User, WhitelistEntry}};

pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
    let sale = &ctx.accounts.sale;
    let user = &mut ctx.accounts.user;

    require!(user.purchased_amount + amount <= sale.max_per_wallet, CustomError::MaxPurchaseLimitReached);
    
    let total_price = amount.checked_mul(sale.token_price).ok_or(ProgramError::InvalidArgument)?;
    require!(ctx.accounts.buyer.lamports() >= total_price, CustomError::InsufficientFunds);

    user.purchased_amount += amount;
    Ok(())
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    buyer: Signer<'info>,
    #[account(
        mut,
        seeds=[b"sale", sale.authority.key().as_ref(), sale.whitelist.as_ref()],
        bump = sale.bump
    )]
    sale: Account<'info, Sale>,
    #[account(
        seeds=[
            buyer.key().as_ref(),
            sale.whitelist.as_ref()
        ],
        bump = whitelist_entry.bump
    )]
    whitelist_entry: Account<'info, WhitelistEntry>,
    #[account(
        init_if_needed,
        payer=buyer,
        space=User::SIZE,
        seeds=[b"user", buyer.key().as_ref(), sale.key().as_ref()],
        bump
    )]
    user: Account<'info, User>,
    system_program: Program<'info, System>,
}

