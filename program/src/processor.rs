use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::JabberInstruction;

pub mod add_admin_to_group;
pub mod create_group_index;
pub mod create_group_thread;
pub mod create_profile;
pub mod create_subscription;
pub mod create_thread;
pub mod delete_group_message;
pub mod delete_message;
pub mod edit_group_thread;
pub mod remove_admin_from_group;
pub mod send_message;
pub mod send_message_group;
pub mod send_tip;
pub mod set_user_profile;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Beginning processing");
        let instruction = JabberInstruction::try_from_slice(&instruction_data[0..1])
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        msg!("Instruction unpacked");

        match instruction {
            JabberInstruction::CreateProfile => {
                msg!("Instruction: Create user profile");
                let params = create_profile::Params::try_from_slice(&instruction_data[1..])
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                create_profile::process(program_id, accounts, params)?;
            }
            JabberInstruction::CreateThread => {
                msg!("Instruction: Create thread");
                let params = create_thread::Params::try_from_slice(&instruction_data[1..])
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                create_thread::process(program_id, accounts, params)?;
            }
            JabberInstruction::SetUserProfile => {
                msg!("Instruction: Set user profile");
                let params = set_user_profile::Params::try_from_slice(&instruction_data[1..])
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                set_user_profile::process(program_id, accounts, params)?;
            }
            JabberInstruction::SendMessage => {
                msg!("Instruction: Send message");
                let params = send_message::Params::try_from_slice(&instruction_data[1..])
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                send_message::process(program_id, accounts, params)?;
            }
            JabberInstruction::CreateGroupThread => {
                msg!("Instruction: Create group thread");
                let params = create_group_thread::Params::try_from_slice(&instruction_data[1..])
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                create_group_thread::process(program_id, accounts, params)?;
            }
            JabberInstruction::EditGroupThread => {
                msg!("Instruction: Edit group thread");
                let params = edit_group_thread::Params::try_from_slice(&instruction_data[1..])
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                edit_group_thread::process(program_id, accounts, params)?;
            }
            JabberInstruction::SendMessageGroup => {
                msg!("Instruction: send message to group");
                let params = send_message_group::Params::try_from_slice(&instruction_data[1..])
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                send_message_group::process(program_id, accounts, params)?
            }
            JabberInstruction::AddAdminToGroup => {
                msg!("Instruction: Add admin to group");
                let params = add_admin_to_group::Params::try_from_slice(&instruction_data[1..])
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                add_admin_to_group::process(program_id, accounts, params)?;
            }
            JabberInstruction::RemoveAdminFromGroup => {
                msg!("Instruction: Remove admin from group");
                let params =
                    remove_admin_from_group::Params::try_from_slice(&instruction_data[1..])
                        .map_err(|_| ProgramError::InvalidInstructionData)?;
                remove_admin_from_group::process(program_id, accounts, params)?;
            }
            JabberInstruction::CreateGroupIndex => {
                msg!("Instruction: Create group thread index");
                let params = create_group_index::Params::try_from_slice(&instruction_data[1..])
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                create_group_index::process(program_id, accounts, params)?;
            }
            JabberInstruction::DeleteMessage => {
                msg!("Instruction: Delete message");
                let params = delete_message::Params::try_from_slice(&instruction_data[1..])
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                delete_message::process(program_id, accounts, params)?;
            }
            JabberInstruction::DeleteGroupMessage => {
                msg!("Instruction: Delete group message");
                let params = delete_group_message::Params::try_from_slice(&instruction_data[1..])
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                delete_group_message::process(program_id, accounts, params)?;
            }
            JabberInstruction::SendTip => {
                msg!("Instruction: Send tip");
                let params = send_tip::Params::try_from_slice(&instruction_data[1..])
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                send_tip::process(program_id, accounts, params)?;
            }
            JabberInstruction::CreateSubscription => {
                msg!("Instruction: Create subscription");
                let params = create_subscription::Params::try_from_slice(&instruction_data[1..])
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                create_subscription::process(program_id, accounts, params)?;
            }
        }
        Ok(())
    }
}
