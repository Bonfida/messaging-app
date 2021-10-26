use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Error, Debug, Clone, FromPrimitive)]
pub enum JabberError {
    #[error("Account not generated deterministically")]
    AccountNotDeterministic = 0,
    #[error("Account not Authorized")]
    AccountNotAuthorized = 1,
    #[error("Account not rent exempt")]
    AccountNotRentExempt = 2,
    #[error("Chat thread exists")]
    ChatThreadExists = 3,
    #[error("Profile profile must be owned by the program")]
    WrongProfileOwner = 4,
    #[error("Data type mismatch")]
    DataTypeMismatch,
    #[error("Thread account must be owned by the program")]
    WrongThreadAccountOwner,
    #[error("The system program account is invalid")]
    WrongSystemProgramAccount,
    #[error("The message account must be owned by the program")]
    WrongMessageAccount,
}
impl From<JabberError> for ProgramError {
    fn from(e: JabberError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for JabberError {
    fn type_of() -> &'static str {
        "Jabber Error"
    }
}
