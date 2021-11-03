pub use crate::processor::{
    add_group_admin, create_group_index, create_group_thread, create_profile, create_thread,
    edit_group_thread, remove_group_admin, send_message, send_message_group, set_user_profile,
};
use crate::utils::SOL_VAULT;
use std::{str::FromStr, vec};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum JabberInstruction {
    // Accounts expected by this insctruction
    //
    // | Index | Writable | Signer | Description           |
    // |-------|----------|--------|-----------------------|
    // | 0     | ❌        | ❌      | System program        |
    // | 1     | ✅        | ❌      | Profile account       |
    // | 2     | ✅        | ✅      | Profile account owner |
    // | 3     | ✅        | ✅      | Fee payer             |
    CreateProfile(create_profile::Params),
    // | Index | Writable | Signer | Description    |
    // |-------|----------|--------|----------------|
    // | 0     | ❌        | ❌      | System program |
    // | 1     | ✅        | ❌      | Thread account |
    // | 2     | ✅        | ✅      | Fee payer      |
    CreateThread(create_thread::Params),
    //
    // Accounts expected by this instruction
    //
    // | Index 	| Writable 	| Signer 	| Description          	|
    // |-------	|----------	|--------	|----------------------	|
    // | 0     	| ✅        	| ✅      	| User                 	|
    // | 1     	| ✅        	| ❌      	| User profile account 	|
    SetUserProfile(set_user_profile::Params),
    //
    // Accounts expected by this instruction
    //
    // | Index 	| Writable 	| Signer 	| Description              	|
    // |-------	|----------	|--------	|--------------------------	|
    // | 0     	| ❌        	| ❌      	| System program           	|
    // | 1     	| ✅        	| ✅      	| Sender account           	|
    // | 2     	| ✅        	| ❌      	| Receiver account         	|
    // | 3     	| ✅        	| ❌      	| Thread account         	|
    // | 4     	| ❌       	| ❌      	| Receiver profile        	|
    // | 5     	| ✅        	| ❌      	| Message account          	|
    // | 6     	| ✅        	| ❌      	| SOL vault account       	|
    SendMessage(send_message::Params),
    //
    // Create group thread
    //
    // | Index | Writable | Signer | Description          |
    // |-------|----------|--------|----------------------|
    // | 0     | ❌        | ❌      | System program       |
    // | 1     | ✅        | ❌      | Group thread account |
    // | 2     | ✅        | ✅      | Fee payer            |
    CreateGroupThread(create_group_thread::Params),
    //
    // Edit group thread
    //
    // | Index | Writable | Signer | Description          |
    // |-------|----------|--------|----------------------|
    // | 0     | ✅        | ✅      | Group owner          |
    // | 1     | ✅        | ❌      | Group thread account |
    EditGroupThread(edit_group_thread::Params),
    //
    // Send message to group
    //
    // | Index | Writable | Signer | Description          |
    // |-------|----------|--------|----------------------|
    // | 0     | ❌        | ❌      | System program       |
    // | 1     | ✅        | ✅      | Sender account       |
    // | 2     | ✅        | ❌      | Group thread account |
    // | 3     | ✅        | ❌      | Destination wallet   |
    // | 4     | ✅        | ❌      | Message account      |
    // | 5     | ✅        | ❌      | SOL vault            |
    SendMessageGroup(send_message_group::Params),
    //
    // Add admin to group
    //
    // | Index | Writable | Signer | Description          |
    // |-------|----------|--------|----------------------|
    // | 0     | ✅        | ❌      | Group thread account |
    // | 1     | ✅        | ✅      | Group owner          |
    AddAdminToGroup(add_group_admin::Params),
    //
    // Remove admin from group
    //
    // | Index | Writable | Signer | Description          |
    // |-------|----------|--------|----------------------|
    // | 0     | ✅        | ❌      | Group thread account |
    // | 1     | ✅        | ✅      | Group owner          |
    RemoveAdminGroup(remove_group_admin::Params),
    //
    // Create thread index account
    //
    // | Index | Writable | Signer | Description        |
    // |-------|----------|--------|--------------------|
    // | 0     | ❌        | ❌      | System program     |
    // | 1     | ✅        | ❌      | Group thread index |
    // | 2     | ✅        | ✅      | Fee payer          |
    CreateGroupIndex(create_group_index::Params),
}

pub fn create_profile(
    jabber_program_id: Pubkey,
    profile_account: Pubkey,
    profile_account_owner: Pubkey,
    fee_payer: Pubkey,
    params: create_profile::Params,
) -> Instruction {
    let instruction_data = JabberInstruction::CreateProfile(params);
    let data = instruction_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new(profile_account, false),
        AccountMeta::new(profile_account_owner, true),
        AccountMeta::new(fee_payer, true),
    ];

    Instruction {
        program_id: jabber_program_id,
        accounts,
        data,
    }
}

pub fn create_thread(
    jabber_program_id: Pubkey,
    thread_account: Pubkey,
    fee_payer: Pubkey,
    params: create_thread::Params,
) -> Instruction {
    let instruction_data = JabberInstruction::CreateThread(params);
    let data = instruction_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new(thread_account, false),
        AccountMeta::new(fee_payer, true),
    ];

    Instruction {
        program_id: jabber_program_id,
        accounts,
        data,
    }
}

pub fn set_user_profile(
    jabber_program_id: Pubkey,
    user: Pubkey,
    user_profile_account: Pubkey,
    params: set_user_profile::Params,
) -> Instruction {
    let instruction_data = JabberInstruction::SetUserProfile(params);
    let data = instruction_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new(user, true),
        AccountMeta::new(user_profile_account, false),
    ];

    Instruction {
        program_id: jabber_program_id,
        accounts,
        data,
    }
}

pub fn send_message(
    jabber_program_id: Pubkey,
    sender: Pubkey,
    receiver: Pubkey,
    thread: Pubkey,
    receiver_profile: Pubkey,
    message: Pubkey,
    params: send_message::Params,
) -> Instruction {
    let instruction_data = JabberInstruction::SendMessage(params);
    let data = instruction_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new(sender, true),
        AccountMeta::new(receiver, false),
        AccountMeta::new(thread, false),
        AccountMeta::new_readonly(receiver_profile, false),
        AccountMeta::new(message, false),
        AccountMeta::new(Pubkey::from_str(SOL_VAULT).unwrap(), false),
    ];
    Instruction {
        program_id: jabber_program_id,
        accounts,
        data,
    }
}

pub fn create_group_thread(
    jabber_program_id: Pubkey,
    group_thread: Pubkey,
    fee_payer: Pubkey,
    params: create_group_thread::Params,
) -> Instruction {
    let instruction_data = JabberInstruction::CreateGroupThread(params);
    let data = instruction_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new(group_thread, false),
        AccountMeta::new(fee_payer, true),
    ];

    Instruction {
        program_id: jabber_program_id,
        accounts,
        data,
    }
}

pub fn edit_group_thread(
    jabber_program_id: Pubkey,
    group_owner: Pubkey,
    group_thread: Pubkey,
    params: edit_group_thread::Params,
) -> Instruction {
    let instruction_data = JabberInstruction::EditGroupThread(params);
    let data = instruction_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new(group_owner, true),
        AccountMeta::new(group_thread, false),
    ];

    Instruction {
        program_id: jabber_program_id,
        accounts,
        data,
    }
}

pub fn send_message_group(
    jabber_program_id: Pubkey,
    sender: Pubkey,
    group_thread: Pubkey,
    destination_wallet: Pubkey,
    message: Pubkey,
    params: send_message_group::Params,
) -> Instruction {
    let instruction_data = JabberInstruction::SendMessageGroup(params);
    let data = instruction_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new(sender, true),
        AccountMeta::new(group_thread, false),
        AccountMeta::new(destination_wallet, false),
        AccountMeta::new(message, false),
        AccountMeta::new(Pubkey::from_str(SOL_VAULT).unwrap(), false),
    ];

    Instruction {
        program_id: jabber_program_id,
        accounts,
        data,
    }
}

pub fn add_admin_to_group(
    jabber_program_id: Pubkey,
    group_thread: Pubkey,
    group_owner: Pubkey,
    params: add_group_admin::Params,
) -> Instruction {
    let instruction_data = JabberInstruction::AddAdminToGroup(params);
    let data = instruction_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new(group_thread, false),
        AccountMeta::new(group_owner, true),
    ];

    Instruction {
        program_id: jabber_program_id,
        accounts,
        data,
    }
}

pub fn remove_admin_from_group(
    jabber_program_id: Pubkey,
    group_thread: Pubkey,
    group_owner: Pubkey,
    params: remove_group_admin::Params,
) -> Instruction {
    let instruction_data = JabberInstruction::RemoveAdminGroup(params);
    let data = instruction_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new(group_thread, false),
        AccountMeta::new(group_owner, true),
    ];

    Instruction {
        program_id: jabber_program_id,
        accounts,
        data,
    }
}

pub fn create_group_index(
    jabber_program_id: Pubkey,
    group_thread_index: Pubkey,
    fee_payer: Pubkey,
    params: create_group_index::Params,
) -> Instruction {
    let instruction_data = JabberInstruction::CreateGroupIndex(params);
    let data = instruction_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new(group_thread_index, false),
        AccountMeta::new(fee_payer, true),
    ];

    Instruction {
        program_id: jabber_program_id,
        accounts,
        data,
    }
}
