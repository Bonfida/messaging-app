//! Create a group index for a user
use crate::error::JabberError;
use crate::state::GroupThreadIndex;
use crate::utils::{check_account_key, check_account_owner};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    system_program,
    sysvar::Sysvar,
};

use bonfida_utils::{BorshSize, InstructionsAccount};

#[derive(BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Params {
    pub group_name: String,
    pub group_thread_key: Pubkey,
    pub owner: Pubkey,
}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    /// The system program account
    pub system_program: &'a T,

    /// The group thread index account
    #[cons(writable)]
    pub group_thread_index: &'a T,

    /// The fee payer account
    #[cons(writable, signer)]
    pub fee_payer: &'a T,
}

impl<'a, 'b: 'a> Accounts<'a, AccountInfo<'b>> {
    pub fn parse(
        _program_id: &Pubkey,
        accounts: &'a [AccountInfo<'b>],
    ) -> Result<Self, ProgramError> {
        let accounts_iter = &mut accounts.iter();

        let accounts = Self {
            system_program: next_account_info(accounts_iter)?,
            group_thread_index: next_account_info(accounts_iter)?,
            fee_payer: next_account_info(accounts_iter)?,
        };

        // Check keys
        check_account_key(
            accounts.system_program,
            &system_program::ID,
            JabberError::WrongSystemProgramAccount,
        )?;

        // Check ownership
        check_account_owner(
            accounts.group_thread_index,
            &system_program::ID,
            JabberError::WrongOwner,
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
        group_name,
        group_thread_key,
        owner,
    } = params;

    let (group_thread_index_key, bump) =
        GroupThreadIndex::find_key(group_name.to_string(), group_thread_key, owner, program_id);

    check_account_key(
        accounts.group_thread_index,
        &group_thread_index_key,
        JabberError::AccountNotDeterministic,
    )?;

    let group_thread_index = GroupThreadIndex::new(group_name.clone(), group_thread_key, owner);
    let space = group_thread_index.borsh_len();

    let lamports = Rent::get()?.minimum_balance(space);
    let allocate_account = create_account(
        accounts.fee_payer.key,
        &group_thread_index_key,
        lamports,
        space as u64,
        program_id,
    );

    invoke_signed(
        &allocate_account,
        &[
            accounts.system_program.clone(),
            accounts.fee_payer.clone(),
            accounts.group_thread_index.clone(),
        ],
        &[&[
            GroupThreadIndex::SEED.as_bytes(),
            group_name.as_bytes(),
            &owner.to_bytes(),
            &group_thread_key.to_bytes(),
            &[bump],
        ]],
    )?;

    group_thread_index.save(&mut accounts.group_thread_index.try_borrow_mut_data()?);

    Ok(())
}
