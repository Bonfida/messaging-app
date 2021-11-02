use std::str::FromStr;

use crate::{
    state::MessageType,
    utils::{
        check_account_key, check_account_owner, check_group_message_type, check_rent_exempt,
        check_signer, FEE, SOL_VAULT,
    },
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
use crate::state::{GroupThread, Message};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Params {
    pub kind: MessageType,
    pub message: Vec<u8>,
    pub group_name: String,
}

struct Accounts<'a, 'b: 'a> {
    system_program: &'a AccountInfo<'b>,
    sender: &'a AccountInfo<'b>,
    group_thread: &'a AccountInfo<'b>,
    destination_wallet: &'a AccountInfo<'b>,
    message: &'a AccountInfo<'b>,
    sol_vault: &'a AccountInfo<'b>,
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
            group_thread: next_account_info(accounts_iter)?,
            destination_wallet: next_account_info(accounts_iter)?,
            message: next_account_info(accounts_iter)?,
            sol_vault: next_account_info(accounts_iter)?,
        };
        check_account_key(
            accounts.system_program,
            &system_program::ID,
            JabberError::WrongSystemProgramAccount,
        )?;
        check_signer(accounts.sender)?;
        check_account_owner(
            accounts.group_thread,
            program_id,
            JabberError::WrongThreadAccountOwner,
        )?;
        check_rent_exempt(accounts.group_thread)?;
        check_account_key(
            accounts.sol_vault,
            &Pubkey::from_str(SOL_VAULT).unwrap(),
            JabberError::WrongSolVaultAccount,
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
        kind,
        message,
        group_name,
    } = params;

    let mut group_thread = GroupThread::from_account_info(accounts.group_thread)?;
    let group_thread_key = GroupThread::create_from_destination_wallet_and_name(
        group_name,
        group_thread.owner,
        program_id,
        group_thread.bump,
    );

    check_account_key(
        accounts.group_thread,
        &group_thread_key,
        JabberError::AccountNotDeterministic,
    )?;

    check_account_key(
        accounts.destination_wallet,
        &group_thread.destination_wallet,
        JabberError::WrongDestinationWallet,
    )?;

    check_group_message_type(&group_thread, &kind)?;

    let (message_key, bump) = Message::find_from_keys(
        group_thread.msg_count,
        &group_thread_key,
        &group_thread_key,
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

    invoke_signed(
        &allocate_account,
        &[
            accounts.system_program.clone(),
            accounts.sender.clone(),
            accounts.message.clone(),
        ],
        &[&[
            Message::SEED.as_bytes(),
            group_thread.msg_count.to_string().as_bytes(),
            &accounts.group_thread.key.to_bytes(),
            &accounts.group_thread.key.to_bytes(),
            &[bump],
        ]],
    )?;

    message.save(&mut accounts.message.data.borrow_mut());
    group_thread.increment_msg_count();
    group_thread.save(&mut accounts.group_thread.data.borrow_mut());
    let is_fee_exempt = GroupThread::is_fee_exempt(&group_thread, *accounts.sender.key, None);

    if !is_fee_exempt && group_thread.lamports_per_message > 0 {
        let transfer_fee = (group_thread.lamports_per_message * FEE) / 100;
        let transfer_amount = group_thread.lamports_per_message - transfer_fee;

        let transfer_amount_instruction = transfer(
            accounts.sender.key,
            accounts.destination_wallet.key,
            transfer_amount,
        );
        let transfer_fee_instruction =
            transfer(accounts.sender.key, accounts.sol_vault.key, transfer_fee);

        invoke(
            &transfer_amount_instruction,
            &[
                accounts.system_program.clone(),
                accounts.sender.clone(),
                accounts.destination_wallet.clone(),
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