use crate::utils::{check_account_key, check_account_owner, check_signer};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::error::JabberError;
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
        params: Params,
    ) -> Result<(Self, Message), ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let accounts = Self {
            sender: next_account_info(accounts_iter)?,
            receiver: next_account_info(accounts_iter)?,
            message: next_account_info(accounts_iter)?,
        };
        check_signer(accounts.sender)?;
        check_account_owner(accounts.message, program_id, JabberError::WrongMessageOwner)?;

        let message = Message::from_account_info(accounts.message)?;

        let (expected_message_key, _) = Message::find_key(
            params.message_index,
            accounts.sender.key,
            accounts.receiver.key,
            program_id,
        );

        check_account_key(
            accounts.message,
            &expected_message_key,
            JabberError::AccountNotDeterministic,
        )?;
        check_account_key(
            accounts.sender,
            &message.sender,
            JabberError::AccountNotAuthorized,
        )?;

        Ok((accounts, message))
    }
}

pub(crate) fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    params: Params,
) -> ProgramResult {
    let (accounts, mut message) = Accounts::parse(program_id, accounts, params)?;

    message.kind = MessageType::Deleted;
    message.save(&mut accounts.message.data.borrow_mut());

    let mut message_lamports = accounts.message.lamports.borrow_mut();
    let mut target_lamports = accounts.sender.lamports.borrow_mut();

    **target_lamports += **message_lamports;

    **message_lamports = 0;

    Ok(())
}
