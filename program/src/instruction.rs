pub use crate::processor::{
    add_admin_to_group, create_group_index, create_group_thread, create_profile, create_thread,
    delete_group_message, delete_message, edit_group_thread, remove_admin_from_group, send_message,
    send_message_group, set_user_profile,
};

use bonfida_utils::InstructionsAccount;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{instruction::Instruction, pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum JabberInstruction {
    // 0
    // Accounts expected by this insctruction
    //
    // | Index | Writable | Signer | Description           |
    // |-------|----------|--------|-----------------------|
    // | 0     | ❌        | ❌      | System program        |
    // | 1     | ✅        | ❌      | Profile account       |
    // | 2     | ✅        | ✅      | Profile account owner |
    // | 3     | ✅        | ✅      | Fee payer             |
    CreateProfile,
    // 1
    // | Index | Writable | Signer | Description    |
    // |-------|----------|--------|----------------|
    // | 0     | ❌        | ❌      | System program |
    // | 1     | ✅        | ❌      | Thread account |
    // | 2     | ✅        | ✅      | Fee payer      |
    CreateThread,
    // 2
    //
    // Accounts expected by this instruction
    //
    // | Index 	| Writable 	| Signer 	| Description          	|
    // |-------	|----------	|--------	|----------------------	|
    // | 0     	| ✅        	| ✅      	| User                 	|
    // | 1     	| ✅        	| ❌      	| User profile account 	|
    SetUserProfile,
    // 3
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
    SendMessage,
    // 4
    //
    // Create group thread
    //
    // | Index | Writable | Signer | Description          |
    // |-------|----------|--------|----------------------|
    // | 0     | ❌        | ❌      | System program       |
    // | 1     | ✅        | ❌      | Group thread account |
    // | 2     | ✅        | ✅      | Fee payer            |
    CreateGroupThread,
    // 5
    //
    // Edit group thread
    //
    // | Index | Writable | Signer | Description          |
    // |-------|----------|--------|----------------------|
    // | 0     | ✅        | ✅      | Group owner          |
    // | 1     | ✅        | ❌      | Group thread account |
    EditGroupThread,
    // 6
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
    SendMessageGroup,
    // 7
    //
    // Add admin to group
    //
    // | Index | Writable | Signer | Description          |
    // |-------|----------|--------|----------------------|
    // | 0     | ✅        | ❌      | Group thread account |
    // | 1     | ✅        | ✅      | Group owner          |
    AddAdminToGroup,
    // 8
    //
    // Remove admin from group
    //
    // | Index | Writable | Signer | Description          |
    // |-------|----------|--------|----------------------|
    // | 0     | ✅        | ❌      | Group thread account |
    // | 1     | ✅        | ✅      | Group owner          |
    RemoveAdminFromGroup,
    // 9
    //
    // Create thread index account
    //
    // | Index | Writable | Signer | Description        |
    // |-------|----------|--------|--------------------|
    // | 0     | ❌        | ❌      | System program     |
    // | 1     | ✅        | ❌      | Group thread index |
    // | 2     | ✅        | ✅      | Fee payer          |
    CreateGroupIndex,
    // 10
    //
    // Delete a message
    //
    // | Index | Writable | Signer | Description     |
    // |-------|----------|--------|-----------------|
    // | 0     | ✅        | ✅      | Sender          |
    // | 1     | ❌        | ❌      | Receiver        |
    // | 2     | ✅        | ❌      | Message account |
    DeleteMessage,
    // 11
    //
    // Delete a group message
    //
    // | Index | Writable | Signer | Description     |
    // |-------|----------|--------|-----------------|
    // | 0     | ❌        | ❌      | Group thread    |
    // | 1     | ✅        | ❌      | Message account |
    // | 2     | ✅        | ✅      | Fee payer       |
    DeleteGroupMessage,
}

pub fn create_profile(
    program_id: Pubkey,
    accounts: create_profile::Accounts<Pubkey>,
    params: create_profile::Params,
) -> Instruction {
    accounts.get_instruction(program_id, JabberInstruction::CreateProfile as u8, params)
}

pub fn create_thread(
    program_id: Pubkey,
    accounts: create_thread::Accounts<Pubkey>,
    params: create_thread::Params,
) -> Instruction {
    accounts.get_instruction(program_id, JabberInstruction::CreateThread as u8, params)
}

pub fn set_user_profile(
    program_id: Pubkey,
    accounts: set_user_profile::Accounts<Pubkey>,
    params: set_user_profile::Params,
) -> Instruction {
    accounts.get_instruction(program_id, JabberInstruction::SetUserProfile as u8, params)
}

pub fn send_message(
    program_id: Pubkey,
    accounts: send_message::Accounts<Pubkey>,
    params: send_message::Params,
) -> Instruction {
    accounts.get_instruction(program_id, JabberInstruction::SendMessage as u8, params)
}

pub fn create_group_thread(
    program_id: Pubkey,
    accounts: create_group_thread::Accounts<Pubkey>,
    params: create_group_thread::Params,
) -> Instruction {
    accounts.get_instruction(
        program_id,
        JabberInstruction::CreateGroupThread as u8,
        params,
    )
}

pub fn edit_group_thread(
    program_id: Pubkey,
    accounts: edit_group_thread::Accounts<Pubkey>,
    params: edit_group_thread::Params,
) -> Instruction {
    accounts.get_instruction(program_id, JabberInstruction::EditGroupThread as u8, params)
}

pub fn send_message_group(
    program_id: Pubkey,
    accounts: send_message_group::Accounts<Pubkey>,
    params: send_message_group::Params,
) -> Instruction {
    accounts.get_instruction(
        program_id,
        JabberInstruction::SendMessageGroup as u8,
        params,
    )
}

pub fn add_admin_to_group(
    program_id: Pubkey,
    accounts: add_admin_to_group::Accounts<Pubkey>,
    params: add_admin_to_group::Params,
) -> Instruction {
    accounts.get_instruction(program_id, JabberInstruction::AddAdminToGroup as u8, params)
}

pub fn remove_admin_from_group(
    program_id: Pubkey,
    accounts: remove_admin_from_group::Accounts<Pubkey>,
    params: remove_admin_from_group::Params,
) -> Instruction {
    accounts.get_instruction(
        program_id,
        JabberInstruction::RemoveAdminFromGroup as u8,
        params,
    )
}

pub fn create_group_index(
    program_id: Pubkey,
    accounts: create_group_index::Accounts<Pubkey>,
    params: create_group_index::Params,
) -> Instruction {
    accounts.get_instruction(
        program_id,
        JabberInstruction::CreateGroupIndex as u8,
        params,
    )
}

pub fn delete_message(
    program_id: Pubkey,
    accounts: delete_message::Accounts<Pubkey>,
    params: delete_message::Params,
) -> Instruction {
    accounts.get_instruction(program_id, JabberInstruction::DeleteMessage as u8, params)
}

pub fn delete_group_message(
    program_id: Pubkey,
    accounts: delete_group_message::Accounts<Pubkey>,
    params: delete_group_message::Params,
) -> Instruction {
    accounts.get_instruction(
        program_id,
        JabberInstruction::DeleteGroupMessage as u8,
        params,
    )
}
