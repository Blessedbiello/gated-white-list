use anchor_lang::prelude::*;

#[account]
pub struct WhitelistTokenSale {
    pub authority: Pubkey,
    pub name: String,
}

impl WhitelistTokenSale {
    pub const SIZE: usize = 8 + 32 + 32;
}

#[account]
pub struct WhitelistEntry {
    pub parent: Pubkey,
    pub whitelisted: Pubkey,
    pub bump: u8, // add bump field
}

impl WhitelistEntry {
    pub const SIZE: usize = 8 + 32 + 32 + 1; // update size
}
