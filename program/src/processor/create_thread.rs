//! Create a DM thread between two users
use crate::error::JabberError;
use crate::state::Thread;
use crate::utils::order_keys;
use crate::utils::{check_account_key, check_account_owner};
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
    pub sender_key: Pubkey,
    pub receiver_key: Pubkey,
}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    /// The system program account
    pub system_program: &'a T,

    /// The thread account
    #[cons(writable)]
    pub thread: &'a T,

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
            thread: next_account_info(accounts_iter)?,
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
            accounts.thread,
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
        sender_key,
        receiver_key,
    } = params;

    let (thread_key, bump) = Thread::find_key(&receiver_key, &sender_key, program_id);

    check_account_key(
        accounts.thread,
        &thread_key,
        JabberError::AccountNotDeterministic,
    )?;

    let (key_1, key_2) = order_keys(&receiver_key, &sender_key);
    let current_time = Clock::get()?.unix_timestamp;
    let thread = Thread::new(key_1, key_2, bump, current_time);

    let lamports = Rent::get()?.minimum_balance(thread.borsh_len());

    let allocate_account = create_account(
        accounts.fee_payer.key,
        &thread_key,
        lamports,
        thread.borsh_len() as u64,
        program_id,
    );

    invoke_signed(
        &allocate_account,
        &[
            accounts.system_program.clone(),
            accounts.fee_payer.clone(),
            accounts.thread.clone(),
        ],
        &[&[
            Thread::SEED.as_bytes(),
            &key_1.to_bytes(),
            &key_2.to_bytes(),
            &[bump],
        ]],
    )?;

    thread.save(&mut accounts.thread.try_borrow_mut_data()?);

    Ok(())
}
