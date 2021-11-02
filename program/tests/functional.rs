use jabber::entrypoint::process_instruction;
use jabber::instruction::{
    add_admin_to_group, add_group_admin, create_group_thread, create_profile, create_thread,
    edit_group_thread, remove_admin_from_group, remove_group_admin, send_message,
    send_message_group,
};
use jabber::state::{GroupThread, MessageType};
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

    ////
    // Test groups instruction
    ////

    // Create group

    let (group_thread, _) = GroupThread::find_from_destination_wallet_and_name(
        "group_name".to_string(),
        prg_test_ctx.payer.pubkey(),
        &jabber_program_id,
    );

    let create_group_thread_instruction = create_group_thread(
        jabber_program_id,
        group_thread,
        prg_test_ctx.payer.pubkey(),
        create_group_thread::Params {
            group_name: "group_name".to_string(),
            destination_wallet: prg_test_ctx.payer.pubkey(),
            lamports_per_message: 1_000_000,
            admins: vec![receiver_account.pubkey()],
            owner: prg_test_ctx.payer.pubkey(),
            media_enabled: true,
        },
    );

    sign_send_instructions(
        &mut prg_test_ctx,
        vec![create_group_thread_instruction],
        vec![],
    )
    .await
    .unwrap();

    // Edit group

    let edit_group_thread_instruction = edit_group_thread(
        jabber_program_id,
        prg_test_ctx.payer.pubkey(),
        group_thread,
        edit_group_thread::Params {
            destination_wallet: receiver_account.pubkey(),
            lamports_per_message: 2 * 1_000_000,
            owner: prg_test_ctx.payer.pubkey(),
            media_enabled: false,
        },
    );

    sign_send_instructions(
        &mut prg_test_ctx,
        vec![edit_group_thread_instruction],
        vec![],
    )
    .await
    .unwrap();

    // Send message

    let (group_message, _) =
        Message::find_from_keys(0, &group_thread, &group_thread, &jabber_program_id);

    let send_group_message_instruction = send_message_group(
        jabber_program_id,
        prg_test_ctx.payer.pubkey(),
        group_thread,
        receiver_account.pubkey(),
        group_message,
        send_message_group::Params {
            kind: MessageType::Unencrypted,
            message: "Coucou les gars".to_string().as_bytes().to_vec(),
            group_name: "group_name".to_string(),
        },
    );

    sign_send_instructions(
        &mut prg_test_ctx,
        vec![send_group_message_instruction],
        vec![],
    )
    .await
    .unwrap();

    // Add admin to group

    let add_admin_instruction = add_admin_to_group(
        jabber_program_id,
        group_thread,
        prg_test_ctx.payer.pubkey(),
        add_group_admin::Params {
            admin_address: receiver_account.pubkey(),
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![add_admin_instruction], vec![])
        .await
        .unwrap();

    // Remove admin from group

    let remove_admin_instruction = remove_admin_from_group(
        jabber_program_id,
        group_thread,
        prg_test_ctx.payer.pubkey(),
        remove_group_admin::Params {
            admin_address: receiver_account.pubkey(),
            admin_index: 1,
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![remove_admin_instruction], vec![])
        .await
        .unwrap();
}
