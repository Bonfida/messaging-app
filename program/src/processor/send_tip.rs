use crate::utils::{check_account_key, check_account_owner, check_signer};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};

use crate::error::JabberError;
use crate::state::Profile;
use spl_token::{instruction::transfer, state::Account};

use bonfida_utils::{BorshSize, InstructionsAccount};

#[derive(BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Params {
    pub amount: u64,
}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    pub spl_token_program: &'a T,
    #[cons(writable)]
    pub sender_profile: &'a T,
    #[cons(writable, signer)]
    pub sender: &'a T,
    #[cons(writable)]
    pub receiver_profile: &'a T,
    pub receiver: &'a T,
    #[cons(writable)]
    pub token_source: &'a T,
    #[cons(writable)]
    pub token_destination: &'a T,
}

impl<'a, 'b: 'a> Accounts<'a, AccountInfo<'b>> {
    pub fn parse(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'b>],
    ) -> Result<Self, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let accounts = Self {
            spl_token_program: next_account_info(accounts_iter)?,
            sender_profile: next_account_info(accounts_iter)?,
            sender: next_account_info(accounts_iter)?,
            receiver_profile: next_account_info(accounts_iter)?,
            receiver: next_account_info(accounts_iter)?,
            token_source: next_account_info(accounts_iter)?,
            token_destination: next_account_info(accounts_iter)?,
        };

        // Check keys
        check_account_key(
            accounts.spl_token_program,
            &spl_token::ID,
            JabberError::WrongSplId,
        )?;

        // Check ownership
        check_account_owner(
            accounts.sender_profile,
            program_id,
            JabberError::WrongProfileOwner,
        )?;
        check_account_owner(
            accounts.receiver_profile,
            program_id,
            JabberError::WrongProfileOwner,
        )?;

        // Check signer
        check_signer(accounts.sender)?;

        Ok(accounts)
    }
}

pub(crate) fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    params: Params,
) -> ProgramResult {
    let accounts = Accounts::parse(program_id, accounts)?;
    let Params { amount } = params;

    let (sender_profile_key, _) = Profile::find_key(accounts.sender.key, program_id);
    let (receiver_profile_key, _) = Profile::find_key(accounts.receiver.key, program_id);

    let destination_token_account =
        Account::unpack_from_slice(&accounts.token_destination.data.borrow())?;

    check_account_key(
        accounts.receiver,
        &destination_token_account.owner,
        JabberError::WrongTipReceiver,
    )?;
    check_account_key(
        accounts.sender_profile,
        &sender_profile_key,
        JabberError::AccountNotDeterministic,
    )?;
    check_account_key(
        accounts.receiver_profile,
        &receiver_profile_key,
        JabberError::AccountNotDeterministic,
    )?;

    let mut sender_profile = Profile::from_account_info(accounts.sender_profile)?;
    let mut receiver_profile = Profile::from_account_info(accounts.receiver_profile)?;

    sender_profile.tips_sent += 1;
    receiver_profile.tips_received += 1;

    sender_profile.save(&mut accounts.sender_profile.data.borrow_mut());
    receiver_profile.save(&mut accounts.receiver_profile.data.borrow_mut());

    // Transfer tokens
    let transfer_ix = transfer(
        &spl_token::ID,
        accounts.token_source.key,
        accounts.token_destination.key,
        accounts.sender.key,
        &[],
        amount,
    )?;
    invoke(
        &transfer_ix,
        &[
            accounts.spl_token_program.clone(),
            accounts.token_source.clone(),
            accounts.token_destination.clone(),
            accounts.sender.clone(),
        ],
    )?;

    Ok(())
}
