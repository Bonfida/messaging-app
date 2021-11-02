use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

use crate::error::JabberError;
use crate::state::{
    GroupThread, MessageType, MAX_ADMIN_LEN, MAX_BIO_LENGTH, MAX_GROUP_NAME_LEN, MAX_NAME_LENGTH,
};
use std::cmp::Ordering::Less;

pub const SOL_VAULT: &str = "GcWEQ9K78FV7LEHteFVciYApERk5YvQuFDQPk1yYJVXi";
pub const FEE: u64 = 1;

// Safety verification functions
pub fn check_account_key(
    account: &AccountInfo,
    key: &Pubkey,
    error: JabberError,
) -> Result<(), JabberError> {
    if account.key != key {
        return Err(error);
    }
    Ok(())
}

pub fn check_account_owner(
    account: &AccountInfo,
    owner: &Pubkey,
    error: JabberError,
) -> Result<(), JabberError> {
    if account.owner != owner {
        return Err(error);
    }
    Ok(())
}

pub fn check_signer(account: &AccountInfo) -> ProgramResult {
    if !(account.is_signer) {
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

pub fn check_rent_exempt(account: &AccountInfo) -> ProgramResult {
    let rent = Rent::get()?;
    if !rent.is_exempt(account.lamports(), account.data_len()) {
        return Err(JabberError::AccountNotRentExempt.into());
    }
    Ok(())
}

pub fn check_profile_params(name: &str, bio: &str) -> ProgramResult {
    if bio.len() > MAX_BIO_LENGTH {
        msg!("Bio is too long - max is {}", MAX_BIO_LENGTH);
        return Err(ProgramError::InvalidArgument);
    }
    if name.len() > MAX_NAME_LENGTH {
        msg!("Name is too long - max is {}", MAX_NAME_LENGTH);
        return Err(ProgramError::InvalidArgument);
    }
    Ok(())
}

pub fn order_keys(key_1: &Pubkey, key_2: &Pubkey) -> (Pubkey, Pubkey) {
    let order = key_1.to_string().cmp(&key_2.to_string());
    if order == Less {
        return (*key_1, *key_2);
    }
    (*key_2, *key_1)
}

pub fn check_group_thread_params(group_name: &str, admins: &[Pubkey]) -> ProgramResult {
    if group_name.len() > MAX_GROUP_NAME_LEN {
        msg!("Group name is too long - max is {}", MAX_GROUP_NAME_LEN);
        return Err(ProgramError::InvalidArgument);
    }
    if admins.len() > MAX_ADMIN_LEN {
        msg!("Too many admins - max is {}", MAX_ADMIN_LEN);
        return Err(ProgramError::InvalidArgument);
    }

    Ok(())
}

pub fn check_group_message_type(
    group_thread: &GroupThread,
    message_type: &MessageType,
) -> ProgramResult {
    match (group_thread.media_enabled, message_type) {
        (false, MessageType::EncryptedImage) => Err(JabberError::NonSupportedMessageType.into()),
        (false, MessageType::UnencryptedImage) => Err(JabberError::NonSupportedMessageType.into()),
        _ => Ok(()),
    }
}
