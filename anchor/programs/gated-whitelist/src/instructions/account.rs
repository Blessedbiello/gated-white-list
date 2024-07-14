// use anchor_lang::prelude::*;

// // the "base" whitelist account upon which all whitelist entry account addresses are derived
// #[account]
// pub struct WhitelistTokenSale {
//     // the account that created this whitelist_token_sale
//     pub authority: Pubkey,
//     // the name of the whitelist_token_sale (max 32 chars)
//     pub name: String
// }

// impl WhitelistTokenSale {
//     pub const SIZE: usize = 8 + 32 + 32;
// }

// // a PDA derived from the address of the account to add and the base whitelist
// // defined in create_whitelist::Whitelist
// //
// // Checking if an account address X is whitelisted in whitelist Y
// // involves checking if a WhitelistEntry exists whose address is derived from X and Y
// #[account]
// pub struct WhitelistEntry {
//     // the base whitelist account that this entry is derived from
//     pub parent: Pubkey,
//     // the address that this entry whitelists
//     pub whitelisted: Pubkey,
// }

// impl WhitelistEntry {
//     pub const SIZE: usize = 8 + 32 + 32;
// }