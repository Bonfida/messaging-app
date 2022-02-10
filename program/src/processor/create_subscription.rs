//! Create a subscription
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

use crate::state::Subscription;

use bonfida_utils::{
    checks::{check_account_key, check_account_owner, check_signer},
    BorshSize, InstructionsAccount,
};

#[derive(BorshSerialize, BorshDeserialize, BorshSize)]
pub struct Params {
    pub subscribed_to: Pubkey,
}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    /// The subscription account
    #[cons(writable)]
    pub subscription: &'a T,

    /// Account to which the user subscribes
    #[cons(writable, signer)]
    pub subscriber: &'a T,

    /// The system program account
    pub system_program: &'a T,
}

impl<'a, 'b: 'a> Accounts<'a, AccountInfo<'b>> {
    pub fn parse(
        _program_id: &Pubkey,
        accounts: &'a [AccountInfo<'b>],
    ) -> Result<Self, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let accounts = Self {
            subscription: next_account_info(accounts_iter)?,
            subscriber: next_account_info(accounts_iter)?,
            system_program: next_account_info(accounts_iter)?,
        };

        // Check keys
        check_account_key(accounts.system_program, &system_program::ID)?;

        // Check ownership
        check_account_owner(accounts.subscription, &system_program::ID)?;

        // Check signer
        check_signer(accounts.subscriber)?;

        Ok(accounts)
    }
}

pub(crate) fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    params: Params,
) -> ProgramResult {
    let accounts = Accounts::parse(program_id, accounts)?;
    let Params { subscribed_to } = params;

    let (subscription_key, bump) =
        Subscription::find_key(accounts.subscriber.key, &subscribed_to, program_id);

    check_account_key(accounts.subscription, &subscription_key)?;

    let subscription = Subscription::new(*accounts.subscriber.key, *accounts.subscription.key);

    let space = subscription.borsh_len();
    let lamports = Rent::get()?.minimum_balance(space);

    let allocate_account = create_account(
        accounts.subscriber.key,
        &subscription_key,
        lamports,
        space as u64,
        program_id,
    );

    invoke_signed(
        &allocate_account,
        &[
            accounts.system_program.clone(),
            accounts.subscriber.clone(),
            accounts.subscription.clone(),
        ],
        &[&[
            Subscription::SEED.as_bytes(),
            &accounts.subscriber.key.to_bytes(),
            &subscribed_to.to_bytes(),
            &[bump],
        ]],
    )?;

    subscription.save(&mut accounts.subscription.try_borrow_mut_data()?);

    Ok(())
}
