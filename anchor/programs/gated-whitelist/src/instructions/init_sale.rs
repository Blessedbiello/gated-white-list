use anchor_lang::prelude::*;
use crate::states::{Sale, WhitelistTokenSale};

pub fn init_sale(
    ctx: Context<InitSale>,
    token_price: u64,
    max_per_wallet: u64
) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    sale.token_price = token_price;
    sale.max_per_wallet = max_per_wallet;
    sale.whitelist = ctx.accounts.whitelist.key();
    Ok(())
}

#[derive(Accounts)]
pub struct InitSale<'info> {
    #[account(
        init,
        payer=authority,
        space=Sale::SIZE,
        seeds=[b"sale", authority.key().as_ref(), whitelist.key().as_ref()],
        bump
    )]
    sale: Account<'info, Sale>,
    #[account(mut)]
    authority: Signer<'info>,
    #[account(has_one=authority)]
    whitelist: Account<'info, WhitelistTokenSale>,
    system_program: Program<'info, System>,
}

// use anchor_lang::prelude::*;
// use crate::states::{Sale, Whitelist};

// pub fn init_sale(ctx: Context<InitSale>, token_price: u64, max_per_wallet: u64) -> Result<()> {
//     let sale = &mut ctx.accounts.sale;
//     sale.token_price = token_price;
//     sale.max_per_wallet = max_per_wallet;
//     sale.whitelist = ctx.accounts.whitelist.key();
//     sale.bump = *ctx.bumps.get("sale").unwrap(); // Set the bump value
//     Ok(())
// }

// #[derive(Accounts)]
// pub struct InitSale<'info> {
//     #[account(
//         init,
//         payer = authority,
//         space = Sale::SIZE,
//         seeds = [b"sale", authority.key().as_ref(), whitelist.key().as_ref()],
//         bump
//     )]
//     sale: Account<'info, Sale>,
//     #[account(has_one = authority)]
//     whitelist: Account<'info, Whitelist>,
//     authority: Signer<'info>,
//     system_program: Program<'info, System>,
// }
