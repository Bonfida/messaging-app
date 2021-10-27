use jabber::entrypoint::process_instruction;
use jabber::instruction::{create_profile, create_thread, send_message};
use jabber::state::MessageType;
use jabber::state::{Message, Profile, Thread};
use solana_program::pubkey::Pubkey;
use solana_program_test::{processor, ProgramTest};
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;

pub mod common;

use crate::common::utils::sign_send_instructions;

#[tokio::test]
async fn test_jabber() {
    // Create program and test environment
    let jabber_program_id = Pubkey::new_unique();

    let program_test =
        ProgramTest::new("jabber", jabber_program_id, processor!(process_instruction));

    // Create test context
    let mut prg_test_ctx = program_test.start_with_context().await;

    // Create receiver
    let receiver_account = Keypair::new();

    // Create profile
    let (profile_account, _) =
        Profile::find_from_user_key(&receiver_account.pubkey(), &jabber_program_id);

    let create_profile_instruction = create_profile(
        jabber_program_id,
        profile_account,
        receiver_account.pubkey(),
        prg_test_ctx.payer.pubkey(),
        create_profile::Params {
            name: "Receiver".to_string(),
            bio: "I receive message".to_string(),
            lamports_per_message: 1_000_000_000,
        },
    );

    sign_send_instructions(
        &mut prg_test_ctx,
        vec![create_profile_instruction],
        vec![&receiver_account],
    )
    .await
    .unwrap();

    // Create thread
    let (thread_account, _) = Thread::find_from_users_keys(
        &receiver_account.pubkey(),
        &prg_test_ctx.payer.pubkey(),
        &jabber_program_id,
    );

    let create_thread_instruction = create_thread(
        jabber_program_id,
        thread_account,
        prg_test_ctx.payer.pubkey(),
        create_thread::Params {
            sender_key: prg_test_ctx.payer.pubkey(),
            receiver_key: receiver_account.pubkey(),
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![create_thread_instruction], vec![])
        .await
        .unwrap();

    // Send message
    let (message_account, _) = Message::find_from_keys(
        0,
        &receiver_account.pubkey(),
        &prg_test_ctx.payer.pubkey(),
        &jabber_program_id,
    );
    let send_message_instruction = send_message(
        jabber_program_id,
        prg_test_ctx.payer.pubkey(),
        receiver_account.pubkey(),
        thread_account,
        profile_account,
        message_account,
        send_message::Params {
            kind: MessageType::Unencrypted,
            message: "Test JKnsfdjbgdfuigjbn sdjknfsdjkfndsfjkn"
                .to_string()
                .as_bytes()
                .to_vec(),
        },
    );
    sign_send_instructions(&mut prg_test_ctx, vec![send_message_instruction], vec![])
        .await
        .unwrap();
}
