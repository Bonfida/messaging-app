//! Remove an admin from the group
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
    pub admin_index: u64,
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

        // Check keys

        // Check ownership
        check_account_owner(
            accounts.group_thread,
            program_id,
            JabberError::WrongGroupThreadOwner,
        )?;

        // Check signer
        check_signer(accounts.group_owner)?;

        Ok(accounts)
    }
}

pub(crate) fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    params: Params,
) -> ProgramResult {
    let accounts = Accounts::parse(program_id, accounts)?;
    let Params {
        admin_address,
        admin_index,
    } = params;

    let mut group_thread = GroupThread::from_account_info(accounts.group_thread)?;

    let (expected_group_thread_key, _) = GroupThread::find_key(
        group_thread.group_name.clone(),
        *accounts.group_owner.key,
        program_id,
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

    group_thread.remove_admin(admin_address, admin_index as usize)?;
    group_thread.save(&mut accounts.group_thread.data.borrow_mut());

    Ok(())
}
