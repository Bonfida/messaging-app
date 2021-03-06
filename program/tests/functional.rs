use jab::entrypoint::process_instruction;
use jab::instruction::{
    add_admin_to_group, create_group_index, create_group_thread, create_profile,
    create_subscription, create_thread, delete_group_message, delete_message, edit_group_thread,
    remove_admin_from_group, send_message, send_message_group, send_tip, set_user_profile,
};
use jab::state::{GroupThread, GroupThreadIndex, MessageType, Subscription};
use jab::state::{Message, Profile, Thread};
use jab::utils::SOL_VAULT;
use solana_program::{pubkey::Pubkey, rent::Rent, system_instruction, system_program};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;
use spl_associated_token_account::{create_associated_token_account, get_associated_token_address};
use spl_token::instruction::{initialize_mint, mint_to};

pub mod common;

use crate::common::utils::sign_send_instructions;

#[tokio::test]
async fn test_jab() {
    // Create program and test environment
    let jab_program_id = Pubkey::new_unique();

    let program_test = ProgramTest::new("jab", jab_program_id, processor!(process_instruction));

    // Create test context
    let mut prg_test_ctx = program_test.start_with_context().await;

    // Create receiver
    let receiver_account = Keypair::new();

    // Create profile
    let (profile_account, _) = Profile::find_key(&receiver_account.pubkey(), &jab_program_id);

    let create_profile_ix = create_profile(
        jab_program_id,
        create_profile::Accounts {
            system_program: &system_program::ID,
            profile: &profile_account,
            profile_owner: &receiver_account.pubkey(),
            fee_payer: &prg_test_ctx.payer.pubkey(),
        },
        create_profile::Params {
            picture_hash: "Receiver".to_string(),
            display_domain_name: "Test".to_string(),
            bio: "I receive message".to_string(),
            lamports_per_message: 1_000_000_000,
        },
    );

    sign_send_instructions(
        &mut prg_test_ctx,
        vec![create_profile_ix],
        vec![&receiver_account],
    )
    .await
    .unwrap();

    // Set user profile

    let set_profile_ix = set_user_profile(
        jab_program_id,
        set_user_profile::Accounts {
            profile_owner: &receiver_account.pubkey(),
            profile: &profile_account,
        },
        set_user_profile::Params {
            picture_hash: "Receiver".to_string(),
            display_domain_name: "Test".to_string(),
            bio: "I receive message".to_string(),
            lamports_per_message: 2_000_000_000,
            allow_dm: true,
        },
    );
    sign_send_instructions(
        &mut prg_test_ctx,
        vec![set_profile_ix],
        vec![&receiver_account],
    )
    .await
    .unwrap();

    // Create thread
    let (thread_account, _) = Thread::find_key(
        &receiver_account.pubkey(),
        &prg_test_ctx.payer.pubkey(),
        &jab_program_id,
    );

    let create_thread_ix = create_thread(
        jab_program_id,
        create_thread::Accounts {
            system_program: &system_program::ID,
            thread: &thread_account,
            fee_payer: &prg_test_ctx.payer.pubkey(),
        },
        create_thread::Params {
            sender_key: prg_test_ctx.payer.pubkey(),
            receiver_key: receiver_account.pubkey(),
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![create_thread_ix], vec![])
        .await
        .unwrap();

    // Send message
    let (message_account, _) = Message::find_key(
        0,
        &receiver_account.pubkey(),
        &prg_test_ctx.payer.pubkey(),
        &jab_program_id,
    );

    let send_message_instruction = send_message(
        jab_program_id,
        send_message::Accounts {
            system_program: &system_program::ID,
            sender: &prg_test_ctx.payer.pubkey(),
            receiver: &receiver_account.pubkey(),
            thread: &thread_account,
            receiver_profile: &profile_account,
            message: &message_account,
            sol_vault: &SOL_VAULT,
        },
        send_message::Params {
            replies_to: Pubkey::default(),
            kind: MessageType::UnencryptedText,
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

    let (group_thread, _) = GroupThread::find_key(
        "group_name".to_string(),
        prg_test_ctx.payer.pubkey(),
        &jab_program_id,
    );

    let create_group_thread_ix = create_group_thread(
        jab_program_id,
        create_group_thread::Accounts {
            system_program: &system_program::ID,
            group_thread: &group_thread,
            fee_payer: &prg_test_ctx.payer.pubkey(),
        },
        create_group_thread::Params {
            visible: true,
            group_name: "group_name".to_string(),
            destination_wallet: prg_test_ctx.payer.pubkey(),
            lamports_per_message: 1_000_000,
            admins: vec![receiver_account.pubkey()],
            owner: prg_test_ctx.payer.pubkey(),
            media_enabled: true,
            admin_only: false,
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![create_group_thread_ix], vec![])
        .await
        .unwrap();

    // Edit group
    let edit_group_thread_ix = edit_group_thread(
        jab_program_id,
        edit_group_thread::Accounts {
            group_owner: &prg_test_ctx.payer.pubkey(),
            group_thread: &group_thread,
        },
        edit_group_thread::Params {
            visible: false,
            destination_wallet: receiver_account.pubkey(),
            lamports_per_message: 2 * 1_000_000,
            owner: prg_test_ctx.payer.pubkey(),
            media_enabled: false,
            admin_only: false,
            group_pic_hash: "".to_string(),
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![edit_group_thread_ix], vec![])
        .await
        .unwrap();

    // Send message

    let (group_message, _) = Message::find_key(0, &group_thread, &group_thread, &jab_program_id);

    let send_group_message_ix = send_message_group(
        jab_program_id,
        send_message_group::Accounts {
            system_program: &system_program::ID,
            sender: &prg_test_ctx.payer.pubkey(),
            group_thread: &group_thread,
            destination_wallet: &receiver_account.pubkey(),
            message: &group_message,
            sol_vault: &SOL_VAULT,
        },
        send_message_group::Params {
            kind: MessageType::UnencryptedText,
            message: "Coucou les gars".to_string().as_bytes().to_vec(),
            group_name: "group_name".to_string(),
            admin_index: None,
            replies_to: Pubkey::default(),
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![send_group_message_ix], vec![])
        .await
        .unwrap();

    // Add admin to group
    let add_admin_ix = add_admin_to_group(
        jab_program_id,
        add_admin_to_group::Accounts {
            group_thread: &group_thread,
            group_owner: &prg_test_ctx.payer.pubkey(),
        },
        add_admin_to_group::Params {
            admin_address: receiver_account.pubkey(),
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![add_admin_ix], vec![])
        .await
        .unwrap();

    // Remove admin from group
    let remove_admin_ix = remove_admin_from_group(
        jab_program_id,
        remove_admin_from_group::Accounts {
            group_thread: &group_thread,
            group_owner: &prg_test_ctx.payer.pubkey(),
        },
        remove_admin_from_group::Params {
            admin_address: receiver_account.pubkey(),
            admin_index: 1,
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![remove_admin_ix], vec![])
        .await
        .unwrap();

    // Create group index

    let (group_index, _) = GroupThreadIndex::find_key(
        "group name".to_string(),
        group_thread,
        prg_test_ctx.payer.pubkey(),
        &jab_program_id,
    );
    let create_group_index_ix = create_group_index(
        jab_program_id,
        create_group_index::Accounts {
            system_program: &system_program::ID,
            group_thread_index: &group_index,
            fee_payer: &prg_test_ctx.payer.pubkey(),
        },
        create_group_index::Params {
            group_name: "group name".to_string(),
            group_thread_key: group_thread,
            owner: prg_test_ctx.payer.pubkey(),
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![create_group_index_ix], vec![])
        .await
        .unwrap();

    let (group_index_2, _) = GroupThreadIndex::find_key(
        "group name".to_string(),
        group_thread,
        receiver_account.pubkey(),
        &jab_program_id,
    );

    let create_group_index_ix = create_group_index(
        jab_program_id,
        create_group_index::Accounts {
            system_program: &system_program::ID,
            group_thread_index: &group_index_2,
            fee_payer: &prg_test_ctx.payer.pubkey(),
        },
        create_group_index::Params {
            group_name: "group name".to_string(),
            group_thread_key: group_thread,
            owner: receiver_account.pubkey(),
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![create_group_index_ix], vec![])
        .await
        .unwrap();

    // Delete message
    let delete_message_ix = delete_message(
        jab_program_id,
        delete_message::Accounts {
            sender: &prg_test_ctx.payer.pubkey(),
            receiver: &receiver_account.pubkey(),
            message: &message_account,
        },
        delete_message::Params { message_index: 0 },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![delete_message_ix], vec![])
        .await
        .unwrap();

    // Delete group message
    let delete_group_message_ix = delete_group_message(
        jab_program_id,
        delete_group_message::Accounts {
            group_thread: &group_thread,
            message: &group_message,
            fee_payer: &prg_test_ctx.payer.pubkey(),
        },
        delete_group_message::Params {
            message_index: 0,
            admin_index: Some(0),
            owner: prg_test_ctx.payer.pubkey(),
            group_name: "group_name".to_string(),
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![delete_group_message_ix], vec![])
        .await
        .unwrap();

    // Sent tip

    let mint = Keypair::new();
    let init_mint_ix = vec![
        system_instruction::create_account(
            &prg_test_ctx.payer.pubkey(),
            &mint.pubkey(),
            Rent::default().minimum_balance(82),
            82,
            &spl_token::id(),
        ),
        initialize_mint(
            &spl_token::id(),
            &mint.pubkey(),
            &prg_test_ctx.payer.pubkey(),
            None,
            6,
        )
        .unwrap(),
    ];
    sign_send_instructions(&mut prg_test_ctx, init_mint_ix, vec![&mint])
        .await
        .unwrap();

    let create_sender_token_acc_ix = create_associated_token_account(
        &prg_test_ctx.payer.pubkey(),
        &prg_test_ctx.payer.pubkey(),
        &mint.pubkey(),
    );
    sign_send_instructions(&mut prg_test_ctx, vec![create_sender_token_acc_ix], vec![])
        .await
        .unwrap();

    let create_receiver_token_acc_ix = create_associated_token_account(
        &prg_test_ctx.payer.pubkey(),
        &receiver_account.pubkey(),
        &mint.pubkey(),
    );
    sign_send_instructions(
        &mut prg_test_ctx,
        vec![create_receiver_token_acc_ix],
        vec![],
    )
    .await
    .unwrap();

    let sender_token_acc =
        get_associated_token_address(&prg_test_ctx.payer.pubkey(), &mint.pubkey());
    let receiver_token_acc =
        get_associated_token_address(&receiver_account.pubkey(), &mint.pubkey());

    let mint_ix = mint_to(
        &spl_token::ID,
        &mint.pubkey(),
        &sender_token_acc,
        &prg_test_ctx.payer.pubkey(),
        &[],
        10_000 * 1_000_000,
    )
    .unwrap();

    sign_send_instructions(&mut prg_test_ctx, vec![mint_ix], vec![])
        .await
        .unwrap();

    let (sender_profile, _) = Profile::find_key(&prg_test_ctx.payer.pubkey(), &jab_program_id);

    let create_sender_profile_ix = create_profile(
        jab_program_id,
        create_profile::Accounts {
            system_program: &system_program::ID,
            profile: &sender_profile,
            profile_owner: &prg_test_ctx.payer.pubkey(),
            fee_payer: &prg_test_ctx.payer.pubkey(),
        },
        create_profile::Params {
            picture_hash: "hash".to_string(),
            display_domain_name: "Domain".to_string(),
            bio: "my bio".to_string(),
            lamports_per_message: 0,
        },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![create_sender_profile_ix], vec![])
        .await
        .unwrap();

    let tip_ix = send_tip(
        jab_program_id,
        send_tip::Accounts {
            spl_token_program: &spl_token::ID,
            sender_profile: &sender_profile,
            sender: &prg_test_ctx.payer.pubkey(),
            receiver_profile: &profile_account,
            receiver: &receiver_account.pubkey(),
            token_source: &sender_token_acc,
            token_destination: &receiver_token_acc,
        },
        send_tip::Params {
            amount: 10 * 1_000_000,
        },
    );
    sign_send_instructions(&mut prg_test_ctx, vec![tip_ix], vec![])
        .await
        .unwrap();

    //
    // Create subscription
    //
    let subscribed_to = Keypair::new().pubkey();
    let (subscription_key, _) = Subscription::find_key(
        &prg_test_ctx.payer.pubkey(),
        &subscribed_to,
        &jab_program_id,
    );

    let create_sub_ix = create_subscription(
        jab_program_id,
        create_subscription::Accounts {
            subscription: &subscription_key,
            subscriber: &prg_test_ctx.payer.pubkey(),
            system_program: &system_program::ID,
        },
        create_subscription::Params { subscribed_to },
    );

    sign_send_instructions(&mut prg_test_ctx, vec![create_sub_ix], vec![])
        .await
        .unwrap();
}
