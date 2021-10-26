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

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Params {
    pub name: String,
    pub bio: String,
    pub lamports_per_message: u64,
}

struct Accounts<'a, 'b: 'a> {
    system_program: &'a AccountInfo<'b>,
    profile: &'a AccountInfo<'b>,
    profile_owner: &'a AccountInfo<'b>,
    fee_payer: &'a AccountInfo<'b>,
}

impl<'a, 'b: 'a> Accounts<'a, 'b> {
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
        name,
        bio,
        lamports_per_message,
    } = params;

    check_profile_params(&name, &bio)?;

    let (profile_key, bump) = Profile::find_from_user_key(accounts.profile_owner.key, program_id);

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

    let profile = Profile::new(name, bio, lamports_per_message, bump);
    profile.save(&mut accounts.profile.try_borrow_mut_data()?);

    Ok(())
}
