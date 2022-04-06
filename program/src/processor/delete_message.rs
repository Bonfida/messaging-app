//! Delete a message (DM)
use crate::utils::{check_account_key, check_account_owner, check_signer};
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
}
#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    /// The message sender account
    #[cons(writable, signer)]
    pub sender: &'a T,

    /// The message receiver account
    pub receiver: &'a T,

    /// The message account
    #[cons(writable)]
    pub message: &'a T,
}

impl<'a, 'b: 'a> Accounts<'a, AccountInfo<'b>> {
    pub fn parse(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'b>],
    ) -> Result<Self, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let accounts = Self {
            sender: next_account_info(accounts_iter)?,
            receiver: next_account_info(accounts_iter)?,
            message: next_account_info(accounts_iter)?,
        };

        // Check keys

        // Check ownership
        check_account_owner(accounts.message, program_id, JabError::WrongMessageOwner)?;

        // Check signer
        check_signer(accounts.sender)?;

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

    let (expected_message_key, _) = Message::find_key(
        params.message_index,
        accounts.sender.key,
        accounts.receiver.key,
        program_id,
    );

    check_account_key(
        accounts.message,
        &expected_message_key,
        JabError::AccountNotDeterministic,
    )?;
    check_account_key(
        accounts.sender,
        &message.sender,
        JabError::AccountNotAuthorized,
    )?;

    message.kind = MessageType::Deleted;
    message.save(&mut accounts.message.data.borrow_mut());

    let mut message_lamports = accounts.message.lamports.borrow_mut();
    let mut target_lamports = accounts.sender.lamports.borrow_mut();

    **target_lamports += **message_lamports;

    **message_lamports = 0;

    Ok(())
}
