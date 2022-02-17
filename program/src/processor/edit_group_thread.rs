//! Edit a group thread information
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
    pub visible: bool,
    pub destination_wallet: Pubkey,
    pub lamports_per_message: u64,
    pub owner: Pubkey,
    pub media_enabled: bool,
    pub admin_only: bool,
    pub group_pic_hash: String,
}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    /// The group owner account
    #[cons(writable, signer)]
    pub group_owner: &'a T,

    /// The group thread account
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

    let group_thread = GroupThread::from_account_info(accounts.group_thread)?;

    let (expected_group_thread_key, _) = GroupThread::find_key(
        group_thread.group_name,
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

    let Params {
        visible,
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
    group_thread.visible = visible;

    group_thread.save(&mut accounts.group_thread.data.borrow_mut());

    Ok(())
}
