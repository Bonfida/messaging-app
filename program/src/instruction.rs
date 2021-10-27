pub use crate::processor::{create_profile, create_thread, send_message, set_user_profile};
use crate::utils::SOL_VAULT;
use std::str::FromStr;

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
