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

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Params {
    pub admin_address: Pubkey,
}

struct Accounts<'a, 'b: 'a> {
    group_thread: &'a AccountInfo<'b>,
    group_owner: &'a AccountInfo<'b>,
}

impl<'a, 'b: 'a> Accounts<'a, 'b> {
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

        let expected_group_thread_key = GroupThread::create_from_destination_wallet_and_name(
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
