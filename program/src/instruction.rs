pub use crate::processor::{
    add_admin_to_group, create_group_index, create_group_thread, create_profile,
    create_subscription, create_thread, delete_group_message, delete_message, edit_group_thread,
    remove_admin_from_group, send_message, send_message_group, send_tip, set_user_profile,
};
use bonfida_utils::InstructionsAccount;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{instruction::Instruction, pubkey::Pubkey};
#[derive(BorshSerialize, BorshDeserialize)]
pub enum JabberInstruction {
    /// Create a user Jabber profile
    ///
    /// | Index | Writable | Signer | Description                |
    /// | ------------------------------------------------------ |
    /// | 0     | ❌        | ❌      | The system program account |
    /// | 1     | ✅        | ❌      | The profile account        |
    /// | 2     | ✅        | ✅      | The profile owner account  |
    /// | 3     | ✅        | ✅      | The fee payer account      |
    CreateProfile,
    /// Create a DM thread between two users
    ///
    /// | Index | Writable | Signer | Description                |
    /// | ------------------------------------------------------ |
    /// | 0     | ❌        | ❌      | The system program account |
    /// | 1     | ✅        | ❌      | The thread account         |
    /// | 2     | ✅        | ✅      | The fee payer account      |
    CreateThread,
    /// Edit a Jabber profile information
    ///
    /// | Index | Writable | Signer | Description               |
    /// | ----------------------------------------------------- |
    /// | 0     | ✅        | ✅      | The profile owner account |
    /// | 1     | ✅        | ❌      | The profile account       |
    SetUserProfile,
    /// Send a message (DM)
    ///
    /// | Index | Writable | Signer | Description                  |
    /// | -------------------------------------------------------- |
    /// | 0     | ❌        | ❌      | The system program account   |
    /// | 1     | ✅        | ✅      | The sender account           |
    /// | 2     | ✅        | ❌      | The receiver account         |
    /// | 3     | ✅        | ❌      | The thread account           |
    /// | 4     | ❌        | ❌      | The receiver profile account |
    /// | 5     | ✅        | ❌      | The message account          |
    /// | 6     | ✅        | ❌      | The SOL vault account        |
    SendMessage,
    /// Create a group thread
    ///
    /// | Index | Writable | Signer | Description                |
    /// | ------------------------------------------------------ |
    /// | 0     | ❌        | ❌      | The system program account |
    /// | 1     | ✅        | ❌      | The group thread account   |
    /// | 2     | ✅        | ✅      | The fee payer account      |
    CreateGroupThread,
    /// Edit a group thread information
    ///
    /// | Index | Writable | Signer | Description              |
    /// | ---------------------------------------------------- |
    /// | 0     | ✅        | ✅      | The group owner account  |
    /// | 1     | ✅        | ❌      | The group thread account |
    EditGroupThread,
    /// Send a message to a group
    ///
    /// | Index | Writable | Signer | Description                |
    /// | ------------------------------------------------------ |
    /// | 0     | ❌        | ❌      | The system program account |
    /// | 1     | ✅        | ✅      | The sender account         |
    /// | 2     | ✅        | ❌      | The group thread account   |
    /// | 3     | ✅        | ❌      | The destination wallet     |
    /// | 4     | ✅        | ❌      | The message account        |
    /// | 5     | ✅        | ❌      | The SOL vault account      |
    SendMessageGroup,
    /// Add an admin to the group
    ///
    /// | Index | Writable | Signer | Description              |
    /// | ---------------------------------------------------- |
    /// | 0     | ✅        | ❌      | The group thread account |
    /// | 1     | ✅        | ✅      | The group owner account  |
    AddAdminToGroup,
    /// Remove an admin from the group
    ///
    /// | Index | Writable | Signer | Description              |
    /// | ---------------------------------------------------- |
    /// | 0     | ✅        | ❌      | The group thread account |
    /// | 1     | ✅        | ✅      | The group owner account  |
    RemoveAdminFromGroup,
    /// Create a group index for a user
    ///
    /// | Index | Writable | Signer | Description                    |
    /// | ---------------------------------------------------------- |
    /// | 0     | ❌        | ❌      | The system program account     |
    /// | 1     | ✅        | ❌      | The group thread index account |
    /// | 2     | ✅        | ✅      | The fee payer account          |
    CreateGroupIndex,
    /// Delete a message (DM)
    ///
    /// | Index | Writable | Signer | Description                  |
    /// | -------------------------------------------------------- |
    /// | 0     | ✅        | ✅      | The message sender account   |
    /// | 1     | ❌        | ❌      | The message receiver account |
    /// | 2     | ✅        | ❌      | The message account          |
    DeleteMessage,
    /// Delete a message sent to a group
    ///
    /// | Index | Writable | Signer | Description              |
    /// | ---------------------------------------------------- |
    /// | 0     | ❌        | ❌      | The group thread account |
    /// | 1     | ✅        | ❌      | The message account      |
    /// | 2     | ✅        | ✅      | The fee payer account    |
    DeleteGroupMessage,
    /// Send a tip
    ///
    /// | Index | Writable | Signer | Description                      |
    /// | ------------------------------------------------------------ |
    /// | 0     | ❌        | ❌      | The SPL token program ID         |
    /// | 1     | ✅        | ❌      | The tip sender profile account   |
    /// | 2     | ✅        | ✅      | The tip sender account           |
    /// | 3     | ✅        | ❌      | The tip receiver profile account |
    /// | 4     | ❌        | ❌      | The tip receiver account         |
    /// | 5     | ✅        | ❌      | The token source account         |
    /// | 6     | ✅        | ❌      | The token destination account    |
    SendTip,
    /// Create a subscription
    ///
    /// | Index | Writable | Signer | Description                          |
    /// | ---------------------------------------------------------------- |
    /// | 0     | ✅        | ❌      | The subscription account             |
    /// | 1     | ✅        | ✅      | Account to which the user subscribes |
    /// | 2     | ❌        | ❌      | The system program account           |
    CreateSubscription,
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
pub fn send_tip(
    program_id: Pubkey,
    accounts: send_tip::Accounts<Pubkey>,
    params: send_tip::Params,
) -> Instruction {
    accounts.get_instruction(program_id, JabberInstruction::SendTip as u8, params)
}
pub fn create_subscription(
    program_id: Pubkey,
    accounts: create_subscription::Accounts<Pubkey>,
    params: create_subscription::Params,
) -> Instruction {
    accounts.get_instruction(
        program_id,
        JabberInstruction::CreateSubscription as u8,
        params,
    )
}
