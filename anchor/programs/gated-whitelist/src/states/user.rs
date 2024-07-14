// use anchor_lang::solana_program::pubkey::Pubkey;

// #[derive(Default, Debug)]
// pub struct User {
//     pub wallet_address: Pubkey,
//     pub amount_purchased: u64,
// }

use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub owner: Pubkey,
    pub purchased_amount: u64,
}

impl User {
    pub const SIZE: usize = 8 + 32 + 8;
}

