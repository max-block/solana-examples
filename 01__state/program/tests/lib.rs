use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};

use counter::COUNTER_SEED;
use solana_program::entrypoint::ProgramResult;
use solana_program::hash::Hash;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::program_error::ProgramError;
use solana_program::system_program;
use solana_program::sysvar;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use counter::entrypoint::process_instruction;
use counter::instruction::CounterInstruction;
use counter::state::{Counter, Settings};

struct TestEnv {
    program_id: Pubkey,
    counter_pubkey: Pubkey,
    settings_pubkey: Pubkey,
    admin_keypair: Keypair,
    user_keypair: Keypair,
    banks_client: BanksClient,
    payer: Keypair,
    recent_blockhash: Hash,
}

impl TestEnv {
    async fn new() -> Result<TestEnv, ProgramError> {
        let program_id = Pubkey::from_str("9onZvMzqAFzSHJrLNVWfqLRFFQ5ZCGzNXB4PBxmp6z5Y").unwrap();
        let admin_keypair = Keypair::new();
        let user_keypair = Keypair::new();
        let counter_pubkey = Pubkey::create_with_seed(&user_keypair.pubkey(), COUNTER_SEED, &program_id)?;
        // let settings_pubkey = Pubkey::create_program_address(&[SETTINGS_SEED.as_bytes()], &program_id)?;
        let (settings_pubkey, _) = Settings::get_settings_pubkey_with_bump(&program_id);

        let mut program_test = ProgramTest::new("counter", program_id, processor!(process_instruction));

        program_test.add_account(admin_keypair.pubkey(), Account{lamports: 9999999999999999999, ..Account::default()});

        // Init counter_account
        // let counter_data = Counter { counter: 0, value: 0 }.try_to_vec()?;
        // program_test.add_account(
        //     counter_pubkey,
        //     Account { lamports: 77777, data: counter_data, owner: program_id, ..Account::default() },
        // );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        

        // Init settings account
        let input = CounterInstruction::UpdateSettings {admin: admin_keypair.pubkey().to_bytes(), inc_step: 1, dec_step: 2 }.try_to_vec()?;
        let mut transaction = Transaction::new_with_payer(
            &[Instruction::new_with_bytes(
                program_id,
                &input,
                vec![
                    AccountMeta::new_readonly(admin_keypair.pubkey(), true),
                    AccountMeta::new(settings_pubkey, false),
                    AccountMeta::new_readonly(sysvar::rent::id(), false),
                    AccountMeta::new_readonly(system_program::id(), false),
                ],
            )],
            Some(&admin_keypair.pubkey()),
        );
        transaction.sign(&[&admin_keypair], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        Ok(TestEnv {
            program_id,
            counter_pubkey,
            settings_pubkey,
            admin_keypair,
            user_keypair,
            banks_client,
            payer,
            recent_blockhash,
        })
    }
}

#[tokio::test]
async fn test_process_inc_ok() -> ProgramResult {
    let mut test_env = TestEnv::new().await?;

    let input = CounterInstruction::Inc.try_to_vec()?;
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            test_env.program_id,
            &input,
            vec![
                AccountMeta::new(test_env.user_keypair.pubkey(), true),
                AccountMeta::new(test_env.counter_pubkey, false),
                AccountMeta::new_readonly(test_env.settings_pubkey, false),
            ],
        )],
        Some(&test_env.payer.pubkey()),
    );
    transaction.sign(&[&test_env.payer, &test_env.user_keypair], test_env.recent_blockhash);
    test_env.banks_client.process_transaction(transaction).await.unwrap();

    let counter_account = test_env.banks_client.get_account(test_env.counter_pubkey).await?.unwrap();
    let counter = Counter::try_from_slice(counter_account.data.as_slice())?;
    assert_eq!(counter.value, 7);
    assert_eq!(counter.counter, 1);
    Ok(())
}

// #[tokio::test]
// async fn test_process_inc_wrong_user() -> ProgramResult {
//     let mut test_env = TestEnv::new().await?;

//     let wrong_user = Keypair::new();

//     let input = CounterInstruction::Inc.try_to_vec()?;
//     let mut transaction = Transaction::new_with_payer(
//         &[Instruction::new_with_bytes(
//             test_env.program_id,
//             &input,
//             vec![
//                 AccountMeta::new(wrong_user.pubkey(), true),
//                 AccountMeta::new(test_env.counter_pubkey, false),
//                 AccountMeta::new_readonly(test_env.settings_pubkey, false),
//             ],
//         )],
//         Some(&test_env.payer.pubkey()),
//     );
//     transaction.sign(&[&test_env.payer, &wrong_user], test_env.recent_blockhash);
//     let res = test_env.banks_client.process_transaction(transaction).await;
//     assert!(res.is_err());

//     let counter_account = test_env.banks_client.get_account(test_env.counter_pubkey).await?.unwrap();
//     let counter = Counter::try_from_slice(counter_account.data.as_slice())?;
//     assert_eq!(counter.value, 0);
//     assert_eq!(counter.counter, 0);
//     Ok(())
// }

// #[tokio::test]
// async fn test_process_dec() -> ProgramResult {
//     let mut test_env = TestEnv::new().await?;

//     let input = CounterInstruction::Dec.try_to_vec()?;
//     let mut transaction = Transaction::new_with_payer(
//         &[Instruction::new_with_bytes(
//             test_env.program_id,
//             &input,
//             vec![
//                 AccountMeta::new(test_env.user_keypair.pubkey(), true),
//                 AccountMeta::new(test_env.counter_pubkey, false),
//                 AccountMeta::new_readonly(test_env.settings_pubkey, false),
//             ],
//         )],
//         Some(&test_env.payer.pubkey()),
//     );
//     transaction.sign(&[&test_env.payer, &test_env.user_keypair], test_env.recent_blockhash);
//     test_env.banks_client.process_transaction(transaction).await.unwrap();

//     let counter_account = test_env.banks_client.get_account(test_env.counter_pubkey).await.unwrap().unwrap();
//     let counter = Counter::try_from_slice(counter_account.data.as_slice()).unwrap();
//     assert_eq!(counter.value, -8);
//     assert_eq!(counter.counter, 1);
//     Ok(())
// }

// #[tokio::test]
// async fn test_process_update_settings() -> ProgramResult {
//     let mut test_env = TestEnv::new().await?;

//     let input = CounterInstruction::UpdateSettings { inc_step: 1, dec_step: 2 }.try_to_vec()?;
//     let mut transaction = Transaction::new_with_payer(
//         &[Instruction::new_with_bytes(
//             test_env.program_id,
//             &input,
//             vec![
//                 AccountMeta::new_readonly(test_env.admin_keypair.pubkey(), true),
//                 AccountMeta::new(test_env.settings_pubkey, false),
//                 AccountMeta::new_readonly(sysvar::rent::id(), false),
//                 AccountMeta::new_readonly(system_program::id(), false),
//             ],
//         )],
//         Some(&test_env.payer.pubkey()),
//     );
//     transaction.sign(&[&test_env.payer, &test_env.admin_keypair], test_env.recent_blockhash);
//     test_env.banks_client.process_transaction(transaction).await.unwrap();

//     let settings_account = test_env.banks_client.get_account(test_env.settings_pubkey).await?.unwrap();
//     let settings = Settings::try_from_slice(settings_account.data.as_slice())?;
//     assert_eq!(settings.inc_step, 1);
//     assert_eq!(settings.dec_step, 2);

//     Ok(())
// }
