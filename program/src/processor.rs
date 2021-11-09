use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::JabberInstruction;

pub mod add_group_admin;
pub mod create_group_index;
pub mod create_group_thread;
pub mod create_profile;
pub mod create_thread;
pub mod delete_group_message;
pub mod delete_message;
pub mod edit_group_thread;
pub mod remove_group_admin;
pub mod send_message;
pub mod send_message_group;
pub mod set_user_profile;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Beginning processing");
        let instruction = JabberInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        msg!("Instruction unpacked");

        match instruction {
            JabberInstruction::CreateProfile(params) => {
                msg!("Instruction: Create user profile");
                create_profile::process(program_id, accounts, params)?;
            }
            JabberInstruction::CreateThread(params) => {
                msg!("Instruction: Create thread");
                create_thread::process(program_id, accounts, params)?;
            }
            JabberInstruction::SetUserProfile(params) => {
                msg!("Instruction: Set user profile");
                set_user_profile::process(program_id, accounts, params)?;
            }
            JabberInstruction::SendMessage(params) => {
                msg!("Instruction: Send message");
                send_message::process(program_id, accounts, params)?;
            }
            JabberInstruction::CreateGroupThread(params) => {
                msg!("Instruction: Create group thread");
                create_group_thread::process(program_id, accounts, params)?;
            }
            JabberInstruction::EditGroupThread(params) => {
                msg!("Instruction: Edit group thread");
                edit_group_thread::process(program_id, accounts, params)?;
            }
            JabberInstruction::SendMessageGroup(params) => {
                msg!("Instruction: send message to group");
                send_message_group::process(program_id, accounts, params)?
            }
            JabberInstruction::AddAdminToGroup(params) => {
                msg!("Instruction: Add admin to group");
                add_group_admin::process(program_id, accounts, params)?;
            }
            JabberInstruction::RemoveAdminGroup(params) => {
                msg!("Instruction: Remove admin from group");
                remove_group_admin::process(program_id, accounts, params)?;
            }
            JabberInstruction::CreateGroupIndex(params) => {
                msg!("Instruction: Create group thread index");
                create_group_index::process(program_id, accounts, params)?;
            }
            JabberInstruction::DeleteMessage(params) => {
                msg!("Instruction: Delete message");
                delete_message::process(program_id, accounts, params)?;
            }
            JabberInstruction::DeleteGroupMessage(params) => {
                msg!("Instruction: Delete group message");
                delete_group_message::process(program_id, accounts, params)?;
            }
        }
        Ok(())
    }
}
