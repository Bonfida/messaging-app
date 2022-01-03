//! Create a user Jabber profile
use crate::utils::{check_account_key, check_signer};
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

use crate::error::JabberError;
use crate::state::{Profile, MAX_PROFILE_LEN};
use crate::utils::check_profile_params;

use bonfida_utils::{BorshSize, InstructionsAccount};

#[derive(BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Params {
    pub picture_hash: String,
    pub display_domain_name: String,
    pub bio: String,
    pub lamports_per_message: u64,
}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    /// The system program account
    pub system_program: &'a T,

    /// The profile account
    #[cons(writable)]
    pub profile: &'a T,

    /// The profile owner account
    #[cons(writable, signer)]
    pub profile_owner: &'a T,

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
            profile: next_account_info(accounts_iter)?,
            profile_owner: next_account_info(accounts_iter)?,
            fee_payer: next_account_info(accounts_iter)?,
        };

        check_account_key(
            accounts.system_program,
            &system_program::ID,
            JabberError::WrongSystemProgramAccount,
        )?;
        check_signer(accounts.profile_owner)?;

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
        picture_hash,
        display_domain_name,
        bio,
        lamports_per_message,
    } = params;

    check_profile_params(&picture_hash, &display_domain_name, &bio)?;

    let (profile_key, bump) = Profile::find_key(accounts.profile_owner.key, program_id);

    check_account_key(
        accounts.profile,
        &profile_key,
        JabberError::AccountNotDeterministic,
    )?;

    let lamports = Rent::get()?.minimum_balance(MAX_PROFILE_LEN);
    let allocate_account = create_account(
        accounts.fee_payer.key,
        accounts.profile.key,
        lamports,
        MAX_PROFILE_LEN as u64,
        program_id,
    );

    invoke_signed(
        &allocate_account,
        &[
            accounts.system_program.clone(),
            accounts.fee_payer.clone(),
            accounts.profile.clone(),
        ],
        &[&[
            Profile::SEED.as_bytes(),
            &accounts.profile_owner.key.to_bytes(),
            &[bump],
        ]],
    )?;

    let profile = Profile::new(
        picture_hash,
        display_domain_name,
        bio,
        lamports_per_message,
        bump,
    );
    profile.save(&mut accounts.profile.try_borrow_mut_data()?);

    Ok(())
}
