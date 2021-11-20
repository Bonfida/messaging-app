use crate::error::JabberError;
use crate::processor::Processor;
use num_traits::FromPrimitive;
use solana_program::{
    account_info::AccountInfo, decode_error::DecodeError, entrypoint::ProgramResult, msg,
    program_error::PrintProgramError, pubkey::Pubkey,
};

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

/// The entrypoint to the Jabber program
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Entrypoint");
    if let Err(error) = Processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<JabberError>();
        return Err(error);
    }
    Ok(())
}

impl PrintProgramError for JabberError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            JabberError::AccountNotDeterministic => {
                msg!("Error: Account not generated deterministically")
            }
            JabberError::AccountNotAuthorized => msg!("Error: Account not rent exempt"),
            JabberError::AccountNotRentExempt => msg!("Error: Account not rent exempt"),
            JabberError::ChatThreadExists => {
                msg!("Error: Chat thread exists")
            }
            JabberError::WrongProfileOwner => {
                msg!("Error: User profile must be owned by the program")
            }
            JabberError::DataTypeMismatch => {
                msg!("Error: Data type mismatch")
            }
            JabberError::WrongThreadAccountOwner => {
                msg!("Error: Thread account must be owned by the program")
            }
            JabberError::WrongSystemProgramAccount => {
                msg!("Error: The system program account is invalid")
            }
            JabberError::WrongMessageAccount => {
                msg!("Error: The message account is invalid")
            }
            JabberError::WrongSolVaultAccount => {
                msg!("Error: Wrong SOL vault account")
            }
            JabberError::MaxAdminsReached => {
                msg!("Error: Maximum number of admins reached")
            }
            JabberError::InvalidAdminIndex => {
                msg!("Error: Invalid admin index")
            }
            JabberError::WrongGroupOwner => {
                msg!("Error: Wrong group owner")
            }
            JabberError::WrongGroupThreadOwner => {
                msg!("Error: Group thread must be owned by the program")
            }
            JabberError::NonSupportedMessageType => {
                msg!("Error: Non supported message type")
            }
            JabberError::WrongDestinationWallet => {
                msg!("Error: Wrong destination wallet")
            }
            JabberError::InvalidHashLength => {
                msg!("Error: Invalid hash length")
            }
            JabberError::WrongMessageOwner => {
                msg!("Error: Wrong message owner")
            }
            JabberError::ChatMuted => {
                msg!("Error: Chat is muted")
            }
        }
    }
}
