use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::hash::Hash;
use solana_program::instruction::{AccountMeta, Instruction};
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
    banks_client: BanksClient,
    payer: Keypair,
    recent_blockhash: Hash,
}

impl TestEnv {
    async fn new() -> Self {
        let program_id = Pubkey::new_unique();
        let counter_pubkey = Pubkey::new_unique();
        let settings_pubkey = Pubkey::new_unique();
        let admin_keypair = Keypair::new();
        let mut program_test = ProgramTest::new("counter", program_id, processor!(process_instruction));

        // Init settings_account
        let settings_data = Settings { admin: admin_keypair.pubkey().to_bytes(), inc_step: 7, dec_step: 8 }.try_to_vec().unwrap();
        program_test.add_account(
            settings_pubkey,
            Account { lamports: 77777, data: settings_data, owner: program_id, ..Account::default() },
        );

        // Init counter_account
        let counter_data = Counter { last_user: admin_keypair.pubkey().to_bytes(), value: 0 }.try_to_vec().unwrap();
        program_test.add_account(
            counter_pubkey,
            Account { lamports: 77777, data: counter_data, owner: program_id, ..Account::default() },
        );

        let (banks_client, payer, recent_blockhash) = program_test.start().await;
        return TestEnv { program_id, counter_pubkey, settings_pubkey, admin_keypair, banks_client, payer, recent_blockhash };
    }
}

#[tokio::test]
async fn test_process_inc() {
    let mut test_env = TestEnv::new().await;

    let input = CounterInstruction::Inc.try_to_vec().unwrap();
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            test_env.program_id,
            &input,
            vec![
                AccountMeta::new(test_env.admin_keypair.pubkey(), true),
                AccountMeta::new(test_env.counter_pubkey, false),
                AccountMeta::new_readonly(test_env.settings_pubkey, false),
            ],
        )],
        Some(&test_env.payer.pubkey()),
    );
    transaction.sign(&[&test_env.payer, &test_env.admin_keypair], test_env.recent_blockhash);
    test_env.banks_client.process_transaction(transaction).await.unwrap();

    let counter_account = test_env.banks_client.get_account(test_env.counter_pubkey).await.unwrap().unwrap();
    let counter = Counter::try_from_slice(counter_account.data.as_slice()).unwrap();
    assert_eq!(counter.value, 7);
}

#[tokio::test]
async fn test_process_dec() {
    let mut test_env = TestEnv::new().await;

    let input = CounterInstruction::Dec.try_to_vec().unwrap();
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            test_env.program_id,
            &input,
            vec![
                AccountMeta::new(test_env.admin_keypair.pubkey(), true),
                AccountMeta::new(test_env.counter_pubkey, false),
                AccountMeta::new_readonly(test_env.settings_pubkey, false),
            ],
        )],
        Some(&test_env.payer.pubkey()),
    );
    transaction.sign(&[&test_env.payer, &test_env.admin_keypair], test_env.recent_blockhash);
    test_env.banks_client.process_transaction(transaction).await.unwrap();

    let counter_account = test_env.banks_client.get_account(test_env.counter_pubkey).await.unwrap().unwrap();
    let counter = Counter::try_from_slice(counter_account.data.as_slice()).unwrap();
    assert_eq!(counter.value, -8);
}

#[tokio::test]
async fn test_process_reset() {
    let mut test_env = TestEnv::new().await;

    // First dec the counter
    let input = CounterInstruction::Dec.try_to_vec().unwrap();
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            test_env.program_id,
            &input,
            vec![
                AccountMeta::new_readonly(test_env.admin_keypair.pubkey(), true),
                AccountMeta::new(test_env.counter_pubkey, false),
                AccountMeta::new_readonly(test_env.settings_pubkey, false),
            ],
        )],
        Some(&test_env.payer.pubkey()),
    );
    transaction.sign(&[&test_env.payer, &test_env.admin_keypair], test_env.recent_blockhash);
    test_env.banks_client.process_transaction(transaction).await.unwrap();

    let counter_account = test_env.banks_client.get_account(test_env.counter_pubkey).await.unwrap().unwrap();
    let counter = Counter::try_from_slice(counter_account.data.as_slice()).unwrap();
    assert_eq!(counter.value, -8);

    // Next reset the counter
    let input = CounterInstruction::Reset.try_to_vec().unwrap();
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            test_env.program_id,
            &input,
            vec![
                AccountMeta::new_readonly(test_env.admin_keypair.pubkey(), true),
                AccountMeta::new(test_env.counter_pubkey, false),
                AccountMeta::new_readonly(test_env.settings_pubkey, false),
            ],
        )],
        Some(&test_env.payer.pubkey()),
    );
    transaction.sign(&[&test_env.payer, &test_env.admin_keypair], test_env.recent_blockhash);
    test_env.banks_client.process_transaction(transaction).await.unwrap();

    let counter_account = test_env.banks_client.get_account(test_env.counter_pubkey).await.unwrap().unwrap();
    let counter = Counter::try_from_slice(counter_account.data.as_slice()).unwrap();
    assert_eq!(counter.value, 0);
}

#[tokio::test]
async fn test_process_update_settings() {
    let mut test_env = TestEnv::new().await;

    let input = CounterInstruction::UpdateSettings { inc_step: 1, dec_step: 2 }.try_to_vec().unwrap();
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            test_env.program_id,
            &input,
            vec![
                AccountMeta::new_readonly(test_env.admin_keypair.pubkey(), true),
                AccountMeta::new(test_env.settings_pubkey, false),
            ],
        )],
        Some(&test_env.payer.pubkey()),
    );
    transaction.sign(&[&test_env.payer, &test_env.admin_keypair], test_env.recent_blockhash);
    test_env.banks_client.process_transaction(transaction).await.unwrap();

    let settings_account = test_env.banks_client.get_account(test_env.settings_pubkey).await.unwrap().unwrap();
    let settings = Settings::try_from_slice(settings_account.data.as_slice()).unwrap();
    assert_eq!(settings.inc_step, 1);
    assert_eq!(settings.dec_step, 2);
}
