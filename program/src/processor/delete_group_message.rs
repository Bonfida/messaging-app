//! Delete a message sent to a group
use crate::{
    state::GroupThread,
    utils::{check_account_key, check_account_owner, check_keys, check_names, check_signer},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::error::JabError;
use crate::state::{Message, MessageType};

use bonfida_utils::{BorshSize, InstructionsAccount};

#[derive(BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Params {
    pub message_index: u32,
    pub owner: Pubkey,
    pub admin_index: Option<u64>,
    pub group_name: String,
}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    /// The group thread account
    pub group_thread: &'a T,

    /// The message account
    #[cons(writable)]
    pub message: &'a T,

    /// The fee payer account
    #[cons(writable, signer)]
    pub fee_payer: &'a T,
}

impl<'a, 'b: 'a> Accounts<'a, AccountInfo<'b>> {
    pub fn parse(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'b>],
    ) -> Result<Self, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let accounts = Self {
            group_thread: next_account_info(accounts_iter)?,
            message: next_account_info(accounts_iter)?,
            fee_payer: next_account_info(accounts_iter)?,
        };

        // Check keys

        // Check ownership
        check_account_owner(
            accounts.group_thread,
            program_id,
            JabError::WrongGroupThreadOwner,
        )?;
        check_account_owner(accounts.message, program_id, JabError::WrongMessageOwner)?;

        // Check signer
        check_signer(accounts.fee_payer)?;

        Ok(accounts)
    }
}

pub(crate) fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    params: Params,
) -> ProgramResult {
    let accounts = Accounts::parse(program_id, accounts)?;

    let mut message = Message::from_account_info(accounts.message)?;
    let group_thread = GroupThread::from_account_info(accounts.group_thread)?;

    let (expected_message_key, _) = Message::find_key(
        params.message_index,
        accounts.group_thread.key,
        accounts.group_thread.key,
        program_id,
    );

    let (expected_group_key, _) =
        GroupThread::find_key(params.group_name.to_string(), params.owner, program_id);

    check_keys(&params.owner, &group_thread.owner)?;
    check_names(&params.group_name, &group_thread.group_name)?;

    check_account_key(
        accounts.group_thread,
        &expected_group_key,
        JabError::AccountNotDeterministic,
    )?;
    check_account_key(
        accounts.message,
        &expected_message_key,
        JabError::AccountNotDeterministic,
    )?;

    // The message can be deleted by:
    // - The original sender
    // - The owner of the group
    // - An admin of the group

    let is_sender = *accounts.fee_payer.key == message.sender;
    let is_owner = *accounts.fee_payer.key == group_thread.owner;

    let mut is_admin = false;

    if let Some(index) = params.admin_index {
        is_admin = group_thread.admins.get(index as usize).unwrap() == accounts.fee_payer.key;
    }

    if !(is_admin || is_sender || is_owner) {
        return Err(JabError::AccountNotAuthorized.into());
    }

    message.kind = MessageType::Deleted;
    message.save(&mut accounts.message.data.borrow_mut());

    let mut message_lamports = accounts.message.lamports.borrow_mut();
    let mut target_lamports = accounts.fee_payer.lamports.borrow_mut();

    **target_lamports += **message_lamports;

    **message_lamports = 0;

    Ok(())
}
