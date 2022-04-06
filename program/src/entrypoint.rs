use crate::error::JabError;
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

/// The entrypoint to the Jab program
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Entrypoint");
    if let Err(error) = Processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<JabError>();
        return Err(error);
    }
    Ok(())
}

impl PrintProgramError for JabError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            JabError::AccountNotDeterministic => {
                msg!("Error: Account not generated deterministically")
            }
            JabError::AccountNotAuthorized => msg!("Error: Account not rent exempt"),
            JabError::AccountNotRentExempt => msg!("Error: Account not rent exempt"),
            JabError::ChatThreadExists => {
                msg!("Error: Chat thread exists")
            }
            JabError::WrongProfileOwner => {
                msg!("Error: User profile must be owned by the program")
            }
            JabError::DataTypeMismatch => {
                msg!("Error: Data type mismatch")
            }
            JabError::WrongThreadAccountOwner => {
                msg!("Error: Thread account must be owned by the program")
            }
            JabError::WrongSystemProgramAccount => {
                msg!("Error: The system program account is invalid")
            }
            JabError::WrongMessageAccount => {
                msg!("Error: The message account is invalid")
            }
            JabError::WrongSolVaultAccount => {
                msg!("Error: Wrong SOL vault account")
            }
            JabError::MaxAdminsReached => {
                msg!("Error: Maximum number of admins reached")
            }
            JabError::InvalidAdminIndex => {
                msg!("Error: Invalid admin index")
            }
            JabError::WrongGroupOwner => {
                msg!("Error: Wrong group owner")
            }
            JabError::WrongGroupThreadOwner => {
                msg!("Error: Group thread must be owned by the program")
            }
            JabError::NonSupportedMessageType => {
                msg!("Error: Non supported message type")
            }
            JabError::WrongDestinationWallet => {
                msg!("Error: Wrong destination wallet")
            }
            JabError::InvalidHashLength => {
                msg!("Error: Invalid hash length")
            }
            JabError::WrongMessageOwner => {
                msg!("Error: Wrong message owner")
            }
            JabError::ChatMuted => {
                msg!("Error: Chat is muted")
            }
            JabError::WrongSplId => {
                msg!("Error: Wrong SPL token program ID")
            }
            JabError::WrongTipReceiver => {
                msg!("Error: Wrong tip receiver")
            }
            JabError::DmClosed => {
                msg!("Error: Receiver does not allow direct messages")
            }
            JabError::WrongOwner => {
                msg!("Error: Wrong account owner")
            }
        }
    }
}
