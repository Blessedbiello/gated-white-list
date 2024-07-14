// use anchor_lang::prelude::*;

// #[error_code]
// pub enum CustomError {
//     #[msg("The account is not whitelisted.")]
//     NotWhitelisted,
//     #[msg("The purchase exceeds the maximum allowed per wallet.")]
//     ExceedsMaxPerWallet,
//     #[msg("Arithmetic overflow occurred.")]
//     Overflow,
// }

use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Max purchase limit reached")]
    MaxPurchaseLimitReached,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
