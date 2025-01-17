use anchor_lang::prelude::*;
// use crate::instructions::account;
use crate::states::account;

pub fn init_whitelist(
    ctx: Context<InitWhitelist>,
    name: String // user-provided name of the whitelist
) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;
    // add the account that created this whitelist as the whitelist admin
    whitelist.authority = *ctx.accounts.authority.signer_key().unwrap();
    whitelist.name = name;
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitWhitelist<'info> {
    #[account(
        init,
        payer=authority,
        space=account::WhitelistTokenSale::SIZE,
        // use the authority and whitelist name as seeds for the PDA
        seeds=[authority.key().as_ref(), name.as_bytes()],
        bump
    )]
    whitelist: Account<'info, account::WhitelistTokenSale>,

    #[account(mut)]
    authority: Signer<'info>, // the account sending (and signing) this transaction
    system_program: Program<'info, System>,
}