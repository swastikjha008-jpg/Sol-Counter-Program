use borsh::{BorshDeserialize, BorshSerialize};
use sol_counter_program::{counter_contract, CounterAccount, InstructionType};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};

fn setup() -> (ProgramTest, Pubkey, Pubkey) {
    let program_id = Pubkey::new_unique();
    let counter_key = Pubkey::new_unique();
    let mut pt = ProgramTest::new(
        "sol_counter_program",
        program_id,
        processor!(counter_contract),
    );
    pt.add_account(
        counter_key,
        Account {
            lamports: 1_000_000,
            data: vec![0u8; 4],
            owner: program_id,
            executable: false,
            rent_epoch: 0,
        },
    );
    (pt, program_id, counter_key)
}

async fn send(
    banks: &mut BanksClient,
    payer: &solana_sdk::signature::Keypair,
    hash: solana_sdk::hash::Hash,
    program_id: Pubkey,
    counter: Pubkey,
    ix: InstructionType,
) -> Result<(), BanksClientError> {
    let instruction = Instruction::new_with_bytes(
        program_id,
        &ix.try_to_vec().unwrap(),
        vec![AccountMeta::new(counter, false)],
    );
    let tx = Transaction::new_signed_with_payer(&[instruction], Some(&payer.pubkey()), &[payer], hash);
    banks.process_transaction(tx).await
}

#[tokio::test]
async fn increment_then_decrement() {
    let (pt, program_id, counter) = setup();
    let (mut banks, payer, hash) = pt.start().await;

    send(&mut banks, &payer, hash, program_id, counter, InstructionType::Increment(10)).await.unwrap();
    let acc = banks.get_account(counter).await.unwrap().unwrap();
    assert_eq!(CounterAccount::try_from_slice(&acc.data).unwrap().count, 10);

    let hash = banks.get_latest_blockhash().await.unwrap();
    send(&mut banks, &payer, hash, program_id, counter, InstructionType::Decrement(4)).await.unwrap();
    let acc = banks.get_account(counter).await.unwrap().unwrap();
    assert_eq!(CounterAccount::try_from_slice(&acc.data).unwrap().count, 6);
}

#[tokio::test]
async fn underflow_fails() {
    let (pt, program_id, counter) = setup();
    let (mut banks, payer, hash) = pt.start().await;
    let res = send(&mut banks, &payer, hash, program_id, counter, InstructionType::Decrement(1)).await;
    assert!(res.is_err());
}
