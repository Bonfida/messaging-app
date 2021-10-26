use crate::{
    state::MessageType,
    utils::{check_account_key, check_account_owner, check_rent_exempt, check_signer, order_keys},
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

use crate::error::JabberError;
use crate::state::{Message, Profile, Thread};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Params {
    pub kind: MessageType,
    pub message: Vec<u8>,
}

struct Accounts<'a, 'b: 'a> {
    system_program: &'a AccountInfo<'b>,
    sender: &'a AccountInfo<'b>,
    receiver: &'a AccountInfo<'b>,
    thread: &'a AccountInfo<'b>,
    receiver_profile: &'a AccountInfo<'b>,
    message: &'a AccountInfo<'b>,
}

impl<'a, 'b: 'a> Accounts<'a, 'b> {
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
        };
        check_account_key(
            accounts.system_program,
            &system_program::ID,
            JabberError::WrongSystemProgramAccount,
        )?;
        check_signer(accounts.sender)?;
        if accounts.sender.key == accounts.receiver.key {
            return Err(ProgramError::InvalidArgument);
        }
        check_account_owner(
            accounts.thread,
            program_id,
            JabberError::WrongThreadAccountOwner,
        )?;

        check_rent_exempt(accounts.thread)?;

        Ok(accounts)
    }
}

pub(crate) fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    params: Params,
) -> ProgramResult {
    let accounts = Accounts::parse(program_id, accounts)?;

    let Params { kind, message } = params;

    let mut thread = Thread::from_account_info(&accounts.thread)?;
    let thread_key = Thread::create_from_user_keys(
        accounts.sender.key,
        accounts.receiver.key,
        program_id,
        thread.bump,
    );
    check_account_key(
        accounts.thread,
        &thread_key,
        JabberError::AccountNotDeterministic,
    )?;

    let (message_key, bump) = Message::find_from_keys(
        thread.msg_count,
        accounts.sender.key,
        accounts.receiver.key,
        program_id,
    );

    check_account_key(
        accounts.message,
        &message_key,
        JabberError::AccountNotDeterministic,
    )?;

    let now = Clock::get()?.unix_timestamp;
    let message = Message::new(kind, now, message, *accounts.sender.key);
    let message_len = message.get_len();
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
    thread.increment_msg_count();
    thread.save(&mut accounts.thread.data.borrow_mut());

    // Transfer lamports if receiver profile exists
    if !accounts.receiver_profile.data_is_empty() {
        check_account_owner(
            accounts.receiver_profile,
            program_id,
            JabberError::WrongProfileOwner,
        )?;
        let profile = Profile::from_account_info(accounts.receiver_profile)?;
        let transfer = transfer(
            accounts.sender.key,
            accounts.receiver.key,
            profile.lamports_per_message,
        );
        invoke(
            &transfer,
            &[
                accounts.system_program.clone(),
                accounts.sender.clone(),
                accounts.receiver.clone(),
            ],
        )?;
    }

    Ok(())
}
