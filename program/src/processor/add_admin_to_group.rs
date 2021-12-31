use crate::error::JabberError;
use crate::state::GroupThread;
use crate::utils::{check_account_key, check_account_owner, check_signer};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use bonfida_utils::{BorshSize, InstructionsAccount};

#[derive(BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Params {
    pub admin_address: Pubkey,
}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    /// The group thread account
    #[cons(writable)]
    pub group_thread: &'a T,

    /// The group owner account
    #[cons(writable, signer)]
    pub group_owner: &'a T,
}

impl<'a, 'b: 'a> Accounts<'a, AccountInfo<'b>> {
    pub fn parse(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'b>],
    ) -> Result<Self, ProgramError> {
        let accounts_iter = &mut accounts.iter();

        let accounts = Self {
            group_thread: next_account_info(accounts_iter)?,
            group_owner: next_account_info(accounts_iter)?,
        };

        check_signer(accounts.group_owner)?;
        check_account_owner(
            accounts.group_thread,
            program_id,
            JabberError::WrongGroupThreadOwner,
        )?;

        if accounts.group_thread.data_is_empty() {
            return Err(ProgramError::UninitializedAccount);
        }

        let group_thread = GroupThread::from_account_info(accounts.group_thread)?;

        let expected_group_thread_key = GroupThread::create_key(
            group_thread.group_name,
            group_thread.owner,
            program_id,
            group_thread.bump,
        );

        check_account_key(
            accounts.group_thread,
            &expected_group_thread_key,
            JabberError::AccountNotDeterministic,
        )?;

        check_account_key(
            accounts.group_owner,
            &group_thread.owner,
            JabberError::WrongGroupOwner,
        )?;

        Ok(accounts)
    }
}

pub(crate) fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    params: Params,
) -> ProgramResult {
    let accounts = Accounts::parse(program_id, accounts)?;
    let Params { admin_address } = params;

    let mut group_thread = GroupThread::from_account_info(accounts.group_thread)?;
    group_thread.add_admin(admin_address)?;
    group_thread.save(&mut accounts.group_thread.data.borrow_mut());

    Ok(())
}
