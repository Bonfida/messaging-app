use crate::utils::{check_account_key, check_account_owner, check_hash_len, check_signer};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::error::JabberError;
use crate::state::GroupThread;

use bonfida_utils::{BorshSize, InstructionsAccount};

#[derive(BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Params {
    pub destination_wallet: Pubkey,
    pub lamports_per_message: u64,
    pub owner: Pubkey,
    pub media_enabled: bool,
    pub group_pic_hash: Option<String>,
    pub admin_only: bool,
}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    #[cons(writable, signer)]
    pub group_owner: &'a T,
    #[cons(writable)]
    pub group_thread: &'a T,
}

impl<'a, 'b: 'a> Accounts<'a, AccountInfo<'b>> {
    pub fn parse(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'b>],
    ) -> Result<Self, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let accounts = Self {
            group_owner: next_account_info(accounts_iter)?,
            group_thread: next_account_info(accounts_iter)?,
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

    let Params {
        destination_wallet,
        lamports_per_message,
        owner,
        media_enabled,
        group_pic_hash,
        admin_only,
    } = params;

    check_hash_len(&group_pic_hash)?;

    let mut group_thread = GroupThread::from_account_info(accounts.group_thread)?;

    group_thread.lamports_per_message = lamports_per_message;
    group_thread.destination_wallet = destination_wallet;
    group_thread.owner = owner;
    group_thread.media_enabled = media_enabled;
    group_thread.group_pic_hash = group_pic_hash;
    group_thread.admin_only = admin_only;

    group_thread.save(&mut accounts.group_thread.data.borrow_mut());

    Ok(())
}
