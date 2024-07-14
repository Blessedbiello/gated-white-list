// use anchor_lang::solana_program::pubkey::Pubkey;

// #[derive(Default, Debug)]
// pub struct Sale {
//     pub authority: Pubkey,
//     pub token_mint: Pubkey,
//     pub price: u64,
//     pub purchase_limit: u64,
//     pub total_supply: u64,
//     pub remaining_supply: u64,
// }

// use anchor_lang::prelude::*;

// #[account]
// pub struct Sale {
//     pub authority: Pubkey,
//     pub token_price: u64,
//     pub max_per_wallet: u64,
// }

// impl Sale {
//     pub const LEN: usize = 32 + 8 + 8; // Size of authority (Pubkey) + token_price (u64) + max_per_wallet (u64)
// }

use anchor_lang::prelude::*;

#[account]
pub struct Sale {
    pub authority: Pubkey,     
    pub token_price: u64,
    pub max_per_wallet: u64,
    pub whitelist: Pubkey,
    pub bump: u8, // add bump field
}

impl Sale {
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 32 + 1; // update size to account for authority
}
