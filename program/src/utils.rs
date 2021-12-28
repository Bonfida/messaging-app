use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

use crate::error::JabberError;
use crate::state::{
    GroupThread, MessageType, MAX_ADMIN_LEN, MAX_BIO_LENGTH, MAX_GROUP_NAME_LEN, MAX_HASH_LEN,
    MAX_NAME_LENGTH,
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

pub fn check_profile_params(
    picture_hash: &str,
    display_domain_name: &str,
    bio: &str,
) -> ProgramResult {
    if bio.len() > MAX_BIO_LENGTH {
        msg!("Bio is too long - max is {}", MAX_BIO_LENGTH);
        return Err(ProgramError::InvalidArgument);
    }
    if picture_hash.len() > MAX_NAME_LENGTH {
        msg!("Hash is too long - max is {}", MAX_NAME_LENGTH);
        return Err(ProgramError::InvalidArgument);
    }
    if display_domain_name.len() > MAX_NAME_LENGTH {
        msg!("Domain is too long - max is {}", MAX_NAME_LENGTH);
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
        (false, MessageType::EncryptedMedia) => Err(JabberError::NonSupportedMessageType.into()),
        (false, MessageType::UnencryptedMedia) => Err(JabberError::NonSupportedMessageType.into()),
        _ => Ok(()),
    }
}

pub fn check_admin_only(
    group_thread: &GroupThread,
    address: &Pubkey,
    admin_index: Option<usize>,
) -> ProgramResult {
    if !group_thread.admin_only || &group_thread.owner == address {
        return Ok(());
    }
    let admin_index = admin_index.unwrap();
    let is_admin = group_thread.admins.get(admin_index).unwrap() == address;
    if !is_admin {
        return Err(JabberError::ChatMuted.into());
    }
    Ok(())
}

pub fn check_hash_len(hash: &Option<String>) -> ProgramResult {
    let too_long = match hash {
        Some(hash) => hash.len() > MAX_HASH_LEN,
        None => false,
    };

    if too_long {
        return Err(JabberError::InvalidHashLength.into());
    }
    Ok(())
}

pub fn check_keys(key_1: &Pubkey, key_2: &Pubkey) -> ProgramResult {
    if key_1 != key_2 {
        msg!("+ Keys are not the same");
        return Err(ProgramError::InvalidArgument);
    }
    Ok(())
}

pub fn check_names(name_1: &str, name_2: &str) -> ProgramResult {
    if name_1 != name_2 {
        msg!("+ names are not the same");
        return Err(ProgramError::InvalidArgument);
    }
    Ok(())
}

#[test]
fn test() {
    use std::str::FromStr;
    assert!(check_names(&"name_1".to_string(), &"name_2".to_string()).is_err());
    assert!(check_names(&"name_1".to_string(), &"name_1".to_string()).is_ok());

    let pubkey_1 = Pubkey::from_str("GJKmBkyZduYq3UxcrAjecBRv1QQ1LqjYoCo8KoqHxb8F").unwrap();
    let pubkey_2 = Pubkey::from_str("otterzZrCX39w8zeRSyXinWgDXS2irJ3GGecVmQJQ5D").unwrap();

    assert!(check_keys(&pubkey_1, &pubkey_2).is_err());
    assert!(check_keys(&pubkey_1, &pubkey_1).is_ok());

    let ordered_keys = order_keys(&pubkey_1, &pubkey_2);

    assert_eq!(ordered_keys, (pubkey_1, pubkey_2));
}
