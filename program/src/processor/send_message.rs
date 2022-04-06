//! Send a message (DM)
use crate::{
    state::MessageType,
    utils::{check_account_key, check_account_owner, check_signer, order_keys, FEE, SOL_VAULT},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::{create_account, transfer},
    system_program,
    sysvar::Sysvar,
};

use crate::error::JabError;
use crate::state::{Message, Profile, Thread};

use bonfida_utils::{BorshSize, InstructionsAccount};

#[derive(BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Params {
    pub kind: MessageType,
    pub replies_to: Pubkey,
    pub message: Vec<u8>,
}

#[derive(InstructionsAccount)]
pub struct Accounts<'a, T> {
    /// The system program account
    pub system_program: &'a T,

    /// The sender account
    #[cons(writable, signer)]
    pub sender: &'a T,

    /// The receiver account
    #[cons(writable)]
    pub receiver: &'a T,

    /// The thread account
    #[cons(writable)]
    pub thread: &'a T,

    /// The receiver profile account
    pub receiver_profile: &'a T,

    /// The message account
    #[cons(writable)]
    pub message: &'a T,

    /// The SOL vault account
    #[cons(writable)]
    pub sol_vault: &'a T,
}

impl<'a, 'b: 'a> Accounts<'a, AccountInfo<'b>> {
    pub fn parse(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'b>],
    ) -> Result<Self, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let accounts = Self {
            system_program: next_account_info(accounts_iter)?,
            sender: next_account_info(accounts_iter)?,
            receiver: next_account_info(accounts_iter)?,
            thread: next_account_info(accounts_iter)?,
            receiver_profile: next_account_info(accounts_iter)?,
            message: next_account_info(accounts_iter)?,
            sol_vault: next_account_info(accounts_iter)?,
        };

        // Check keys
        check_account_key(
            accounts.system_program,
            &system_program::ID,
            JabError::WrongSystemProgramAccount,
        )?;
        check_account_key(
            accounts.sol_vault,
            &SOL_VAULT,
            JabError::WrongSolVaultAccount,
        )?;

        // Check ownership
        check_account_owner(
            accounts.thread,
            program_id,
            JabError::WrongThreadAccountOwner,
        )?;
        check_account_owner(accounts.message, &system_program::ID, JabError::WrongOwner)?;

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

    let Params {
        kind,
        message,
        replies_to,
    } = params;

    let mut thread = Thread::from_account_info(accounts.thread)?;
    let thread_key = Thread::create_key(
        accounts.sender.key,
        accounts.receiver.key,
        program_id,
        thread.bump,
    );
    check_account_key(
        accounts.thread,
        &thread_key,
        JabError::AccountNotDeterministic,
    )?;

    let (message_key, bump) = Message::find_key(
        thread.msg_count,
        accounts.sender.key,
        accounts.receiver.key,
        program_id,
    );

    check_account_key(
        accounts.message,
        &message_key,
        JabError::AccountNotDeterministic,
    )?;

    let now = Clock::get()?.unix_timestamp;
    let message = Message::new(kind, now, message, *accounts.sender.key, replies_to);
    let message_len = message.borsh_len();
    let lamports = Rent::get()?.minimum_balance(message_len);

    let allocate_account = create_account(
        accounts.sender.key,
        &message_key,
        lamports,
        message_len as u64,
        program_id,
    );

    let (key_1, key_2) = order_keys(accounts.sender.key, accounts.receiver.key);

    invoke_signed(
        &allocate_account,
        &[
            accounts.system_program.clone(),
            accounts.sender.clone(),
            accounts.message.clone(),
        ],
        &[&[
            Message::SEED.as_bytes(),
            thread.msg_count.to_string().as_bytes(),
            &key_1.to_bytes(),
            &key_2.to_bytes(),
            &[bump],
        ]],
    )?;

    message.save(&mut accounts.message.data.borrow_mut());

    thread.increment_msg_count(now);
    thread.save(&mut accounts.thread.data.borrow_mut());

    // Transfer lamports if receiver profile exists
    if !accounts.receiver_profile.data_is_empty() {
        check_account_owner(
            accounts.receiver_profile,
            program_id,
            JabError::WrongProfileOwner,
        )?;
        let profile = Profile::from_account_info(accounts.receiver_profile)?;

        if !profile.allow_dm {
            return Err(JabError::DmClosed.into());
        }

        let transfer_fee = profile
            .lamports_per_message
            .checked_mul(FEE)
            .unwrap()
            .checked_div(100)
            .unwrap();
        let transfer_amount = profile
            .lamports_per_message
            .checked_sub(transfer_fee)
            .unwrap();

        let transfer_amount_instruction =
            transfer(accounts.sender.key, accounts.receiver.key, transfer_amount);
        let transfer_fee_instruction =
            transfer(accounts.sender.key, accounts.sol_vault.key, transfer_fee);

        invoke(
            &transfer_amount_instruction,
            &[
                accounts.system_program.clone(),
                accounts.sender.clone(),
                accounts.receiver.clone(),
            ],
        )?;

        invoke(
            &transfer_fee_instruction,
            &[
                accounts.system_program.clone(),
                accounts.sender.clone(),
                accounts.sol_vault.clone(),
            ],
        )?;
    }

    Ok(())
}
