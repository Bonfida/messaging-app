//! Edit a Jabber profile information
use crate::error::JabberError;
use crate::state::Profile;
use crate::utils::{check_account_key, check_account_owner, check_profile_params, check_signer};
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
    pub picture_hash: String,
    pub display_domain_name: String,
    pub bio: String,
    pub lamports_per_message: u64,
    pub allow_dm: bool,
}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    /// The profile owner account
    #[cons(writable, signer)]
    pub profile_owner: &'a T,

    /// The profile account
    #[cons(writable)]
    pub profile: &'a T,
}

impl<'a, 'b: 'a> Accounts<'a, AccountInfo<'b>> {
    pub fn parse(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'b>],
    ) -> Result<Self, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let accounts = Self {
            profile_owner: next_account_info(accounts_iter)?,
            profile: next_account_info(accounts_iter)?,
        };

        check_signer(accounts.profile_owner)?;
        check_account_owner(accounts.profile, program_id, JabberError::WrongProfileOwner)?;

        if accounts.profile.data_is_empty() {
            return Err(ProgramError::UninitializedAccount);
        }

        let (expected_user_profile_key, _) =
            Profile::find_key(accounts.profile_owner.key, program_id);

        check_account_key(
            accounts.profile,
            &expected_user_profile_key,
            JabberError::AccountNotDeterministic,
        )?;

        Ok(accounts)
    }
}

pub(crate) fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    params: Params,
) -> ProgramResult {
    let Params {
        picture_hash,
        display_domain_name,
        bio,
        lamports_per_message,
        allow_dm,
    } = params;

    check_profile_params(&picture_hash, &display_domain_name, &bio)?;

    let accounts = Accounts::parse(program_id, accounts)?;

    let mut profile = Profile::from_account_info(accounts.profile)?;

    profile.lamports_per_message = lamports_per_message;
    profile.bio = bio;
    profile.picture_hash = picture_hash;
    profile.allow_dm = allow_dm;

    profile.save(&mut accounts.profile.data.borrow_mut());

    Ok(())
}
