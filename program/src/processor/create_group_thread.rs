use crate::error::JabberError;
use crate::state::{GroupThread, MAX_GROUP_THREAD_LEN};
use crate::utils::{check_account_key, check_group_thread_params};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
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
    pub destination_wallet: Pubkey,
    pub lamports_per_message: u64,
    pub admins: Vec<Pubkey>,
    pub owner: Pubkey,
    pub media_enabled: bool,
    pub admin_only: bool,
}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    pub system_program: &'a T,
    pub group_thread: &'a T,
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
            group_thread: next_account_info(accounts_iter)?,
            fee_payer: next_account_info(accounts_iter)?,
        };

        check_account_key(
            accounts.system_program,
            &system_program::ID,
            JabberError::WrongSystemProgramAccount,
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
        destination_wallet,
        lamports_per_message,
        admins,
        owner,
        media_enabled,
        admin_only,
    } = params;

    let (group_thread_key, bump) = GroupThread::find_key(group_name.to_string(), owner, program_id);

    check_group_thread_params(&group_name, &admins)?;

    check_account_key(
        accounts.group_thread,
        &group_thread_key,
        JabberError::AccountNotDeterministic,
    )?;

    let lamports = Rent::get()?.minimum_balance(MAX_GROUP_THREAD_LEN);
    let allocate_account = create_account(
        accounts.fee_payer.key,
        &group_thread_key,
        lamports,
        MAX_GROUP_THREAD_LEN as u64,
        program_id,
    );

    invoke_signed(
        &allocate_account,
        &[
            accounts.system_program.clone(),
            accounts.fee_payer.clone(),
            accounts.group_thread.clone(),
        ],
        &[&[
            GroupThread::SEED.as_bytes(),
            group_name.as_bytes(),
            &owner.to_bytes(),
            &[bump],
        ]],
    )?;

    let current_time = Clock::get()?.unix_timestamp;

    let group_thread = GroupThread::new(
        group_name,
        destination_wallet,
        lamports_per_message,
        bump,
        admins,
        owner,
        media_enabled,
        admin_only,
        current_time,
    );

    group_thread.save(&mut accounts.group_thread.try_borrow_mut_data()?);

    Ok(())
}
