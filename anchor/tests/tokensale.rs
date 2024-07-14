// use anchor_lang::prelude::*;
// use anchor_lang::prelude::Pubkey;
// use anchor_lang::solana_program::system_program;
// use anchor_lang::solana_program::program_pack::Pack;
// use anchor_lang::solana_sdk::signature::{Keypair, Signer};
// use anchor_lang::ToAccountInfo;

// use crate::whitelist_token_sale::*;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use anchor_lang::prelude::Pubkey;
//     use anchor_lang::solana_program::system_program;
//     use anchor_lang::solana_program::program_pack::Pack;
//     use anchor_lang::solana_sdk::signature::{Keypair, Signer};
//     use anchor_lang::ToAccountInfo;

//     #[tokio::test]
//     async fn test_whitelist_token_sale() {        // Initialize test environment
//         let program_id = Pubkey::from_str("A1QHrhtGGSMDGaAtXvHwCGLFcABbjVnfuZikLkQipGDn").unwrap();
//         let payer = Keypair::new();
//         let authority = Keypair::new();

//         let mut program_test = ProgramTest::new(
//             "whitelist_token_sale",
//             program_id,
//             processor!(whitelist_token_sale::entry),
//         );

//         program_test.add_account(
//             payer.pubkey(),
//             Account {
//                 lamports: 10_000_000_000,
//                 ..Account::default()
//             },
//         );

//         let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

//         let rent = banks_client.get_rent().await.unwrap();
//         let rent_exempt_balance = rent.minimum_balance(WhitelistTokenSale::SIZE);

//         // Initialize Whitelist
//         let whitelist_name = "TestWhitelist".to_string();
//         let whitelist_seed = &[authority.pubkey().as_ref(), whitelist_name.as_bytes()];
//         let (whitelist_pda, whitelist_bump) = Pubkey::find_program_address(whitelist_seed, &program_id);

//         let init_whitelist_ix = whitelist_token_sale::instruction::init_whitelist(
//             program_id,
//             whitelist_name.clone(),
//             authority.pubkey(),
//         );

//         let transaction = Transaction::new_signed_with_payer(
//             &[init_whitelist_ix],
//             Some(&payer.pubkey()),
//             &[&payer, &authority],
//             recent_blockhash,
//         );

//         banks_client.process_transaction(transaction).await.unwrap();

//         // Add account to whitelist
//         let account_to_add = Keypair::new();
//         let add_to_whitelist_ix = whitelist_token_sale::instruction::add_to_whitelist(
//             program_id,
//             whitelist_pda,
//             account_to_add.pubkey(),
//             authority.pubkey(),
//         );

//         let transaction = Transaction::new_signed_with_payer(
//             &[add_to_whitelist_ix],
//             Some(&payer.pubkey()),
//             &[&payer, &authority],
//             recent_blockhash,
//         );

//         banks_client.process_transaction(transaction).await.unwrap();

//         // Initialize Token Sale
//         let token_price = 1_000_000;
//         let max_per_wallet = 100;
//         let sale_seed = &[b"sale", authority.pubkey().as_ref(), whitelist_pda.as_ref()];
//         let (sale_pda, sale_bump) = Pubkey::find_program_address(sale_seed, &program_id);

//         let init_sale_ix = whitelist_token_sale::instruction::init_sale(
//             program_id,
//             sale_pda,
//             token_price,
//             max_per_wallet,
//             authority.pubkey(),
//             whitelist_pda,
//         );

//         let transaction = Transaction::new_signed_with_payer(
//             &[init_sale_ix],
//             Some(&payer.pubkey()),
//             &[&payer, &authority],
//             recent_blockhash,
//         );

//         banks_client.process_transaction(transaction).await.unwrap();

//         // Buy Tokens
//         let buy_amount = 10;
//         let buy_tokens_ix = whitelist_token_sale::instruction::buy_tokens(
//             program_id,
//             sale_pda,
//             account_to_add.pubkey(),
//             buy_amount,
//         );

//         let transaction = Transaction::new_signed_with_payer(
//             &[buy_tokens_ix],
//             Some(&payer.pubkey()),
//             &[&payer, &account_to_add],
//             recent_blockhash,
//         );

//         banks_client.process_transaction(transaction).await.unwrap();

//         // Verify the purchase
//         let user_seed = &[b"user", account_to_add.pubkey().as_ref(), sale_pda.as_ref()];
//         let (user_pda, _user_bump) = Pubkey::find_program_address(user_seed, &program_id);

//         let user_account = banks_client.get_account(user_pda).await.unwrap().unwrap();
//         let user_data = User::unpack(&user_account.data).unwrap();

//         assert_eq!(user_data.purchased_amount, buy_amount);
//     }
// }

use anchor_lang::prelude::*;
use anchor_lang::prelude::Pubkey;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_sdk::signature::{Keypair, Signer};
use anchor_lang::ToAccountInfo;

use crate::whitelist_token_sale::*;

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;
    use anchor_lang::solana_program::system_program;
    use anchor_lang::solana_program::program_pack::Pack;
    use anchor_lang::solana_sdk::signature::{Keypair, Signer};
    use anchor_lang::ToAccountInfo;
    use solana_program_test::*;
    use solana_sdk::{account::Account, transaction::Transaction};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_whitelist_token_sale() {
        // Initialize test environment
        let program_id = Pubkey::from_str("A1QHrhtGGSMDGaAtXvHwCGLFcABbjVnfuZikLkQipGDn").unwrap();
        let payer = Keypair::new();
        let authority = Keypair::new();

        let mut program_test = ProgramTest::new(
            "whitelist_token_sale",
            program_id,
            processor!(whitelist_token_sale::entry),
        );

        program_test.add_account(
            payer.pubkey(),
            Account {
                lamports: 10_000_000_000,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let rent = banks_client.get_rent().await.unwrap();
        let rent_exempt_balance = rent.minimum_balance(WhitelistTokenSale::SIZE);

        // Initialize Whitelist
        let whitelist_name = "TestWhitelist".to_string();
        let whitelist_seed = &[authority.pubkey().as_ref(), whitelist_name.as_bytes()];
        let (whitelist_pda, whitelist_bump) = Pubkey::find_program_address(whitelist_seed, &program_id);

        let init_whitelist_ix = whitelist_token_sale::instruction::init_whitelist(
            program_id,
            whitelist_name.clone(),
            authority.pubkey(),
        );

        let transaction = Transaction::new_signed_with_payer(
            &[init_whitelist_ix],
            Some(&payer.pubkey()),
            &[&payer, &authority],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Add account to whitelist
        let account_to_add = Keypair::new();
        let add_to_whitelist_ix = whitelist_token_sale::instruction::add_to_whitelist(
            program_id,
            whitelist_pda,
            account_to_add.pubkey(),
            authority.pubkey(),
        );

        let transaction = Transaction::new_signed_with_payer(
            &[add_to_whitelist_ix],
            Some(&payer.pubkey()),
            &[&payer, &authority],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Initialize Token Sale
        let token_price = 1_000_000;
        let max_per_wallet = 100;
        let sale_seed = &[b"sale", authority.pubkey().as_ref(), whitelist_pda.as_ref()];
        let (sale_pda, sale_bump) = Pubkey::find_program_address(sale_seed, &program_id);

        let init_sale_ix = whitelist_token_sale::instruction::init_sale(
            program_id,
            sale_pda,
            token_price,
            max_per_wallet,
            authority.pubkey(),
            whitelist_pda,
        );

        let transaction = Transaction::new_signed_with_payer(
            &[init_sale_ix],
            Some(&payer.pubkey()),
            &[&payer, &authority],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Buy Tokens
        let buy_amount = 10;
        let buy_tokens_ix = whitelist_token_sale::instruction::buy_tokens(
            program_id,
            sale_pda,
            account_to_add.pubkey(),
            buy_amount,
        );

        let transaction = Transaction::new_signed_with_payer(
            &[buy_tokens_ix],
            Some(&payer.pubkey()),
            &[&payer, &account_to_add],
            recent_blockhash,
        );

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify the purchase
        let user_seed = &[b"user", account_to_add.pubkey().as_ref(), sale_pda.as_ref()];
        let (user_pda, _user_bump) = Pubkey::find_program_address(user_seed, &program_id);

        let user_account = banks_client.get_account(user_pda).await.unwrap().unwrap();
        let user_data = User::unpack(&user_account.data).unwrap();

        assert_eq!(user_data.purchased_amount, buy_amount);
    }
}
