use crate::{error::JabberError, utils::order_keys};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, clock::UnixTimestamp, program_error::ProgramError, pubkey::Pubkey,
};

pub const MAX_NAME_LENGTH: usize = 100;
pub const MAX_BIO_LENGTH: usize = 100;
pub const MAX_MSG_LEN: usize = 1_000;

pub const MAX_PROFILE_LEN: usize = 1 + MAX_NAME_LENGTH + MAX_BIO_LENGTH + 8 + 1;

pub const MAX_THREAD_LEN: usize = 1 + 4 + 32 + 32 + 1;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum Tag {
    Uninitialized,
    Profile,
    Thread,
    Message,
    Jabber,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct Profile {
    pub tag: Tag,
    pub picture_hash: String,
    pub bio: String,
    pub lamports_per_message: u64,
    pub bump: u8,
}
impl Profile {
    pub const SEED: &'static str = "profile";

    pub fn find_from_user_key(user_key: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
        let (user_profile_key, bump) = Pubkey::find_program_address(
            &[Profile::SEED.as_bytes(), &user_key.to_bytes()],
            program_id,
        );
        return (user_profile_key, bump);
    }

    pub fn create_from_keys(user_key: &Pubkey, program_id: &Pubkey, bump: u8) -> Pubkey {
        let seeds = &[Profile::SEED.as_bytes(), &user_key.to_bytes(), &[bump]];
        Pubkey::create_program_address(seeds, program_id).unwrap()
    }

    pub fn new(picture_hash: String, bio: String, lamports_per_message: u64, bump: u8) -> Self {
        Self {
            tag: Tag::Profile,
            picture_hash,
            bio,
            lamports_per_message,
            bump,
        }
    }

    pub fn save(&self, mut dst: &mut [u8]) {
        self.serialize(&mut dst).unwrap()
    }

    pub fn from_account_info(a: &AccountInfo) -> Result<Profile, ProgramError> {
        let mut data = &a.data.borrow() as &[u8];
        if data[0] != Tag::Profile as u8 && data[0] != Tag::Uninitialized as u8 {
            return Err(JabberError::DataTypeMismatch.into());
        }
        let result = Profile::deserialize(&mut data)?;
        Ok(result)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct Thread {
    pub tag: Tag,
    pub msg_count: u32,
    pub user_1: Pubkey,
    pub user_2: Pubkey,
    pub bump: u8,
}

impl Thread {
    pub const SEED: &'static str = "thread";

    pub fn new(user_1: Pubkey, user_2: Pubkey, bump: u8) -> Self {
        Self {
            tag: Tag::Thread,
            msg_count: 0,
            user_1,
            user_2,
            bump,
        }
    }

    pub fn find_from_users_keys(
        user_1: &Pubkey,
        user_2: &Pubkey,
        program_id: &Pubkey,
    ) -> (Pubkey, u8) {
        let (key_1, key_2) = order_keys(user_1, user_2);
        let (thread_key, bump) = Pubkey::find_program_address(
            &[
                Thread::SEED.as_bytes(),
                &key_1.to_bytes(),
                &key_2.to_bytes(),
            ],
            program_id,
        );
        return (thread_key, bump);
    }

    pub fn create_from_user_keys(
        user_1: &Pubkey,
        user_2: &Pubkey,
        program_id: &Pubkey,
        bump: u8,
    ) -> Pubkey {
        let (key_1, key_2) = order_keys(user_1, user_2);
        let seeds = &[
            Thread::SEED.as_bytes(),
            &key_1.to_bytes(),
            &key_2.to_bytes(),
            &[bump],
        ];
        Pubkey::create_program_address(seeds, program_id).unwrap()
    }

    pub fn save(&self, mut dst: &mut [u8]) {
        self.serialize(&mut dst).unwrap()
    }

    pub fn from_account_info(a: &AccountInfo) -> Result<Thread, ProgramError> {
        let mut data = &a.data.borrow() as &[u8];
        if data[0] != Tag::Thread as u8 && data[0] != Tag::Uninitialized as u8 {
            return Err(JabberError::DataTypeMismatch.into());
        }
        let result = Thread::deserialize(&mut data)?;
        Ok(result)
    }

    pub fn increment_msg_count(&mut self) {
        self.msg_count += 1;
    }
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum MessageType {
    Encrypted,
    Unencrypted,
    EncryptedImage,
    UnencryptedImage
}

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub struct Message {
    pub tag: Tag,
    pub kind: MessageType,
    pub timestamp: UnixTimestamp,
    pub msg: Vec<u8>,
    pub sender: Pubkey,
}

impl Message {
    pub const SEED: &'static str = "message";

    pub fn get_len(&self) -> usize {
        1 + 1 + 8 + self.msg.len() + 4 + 32
    }

    pub fn new(kind: MessageType, timestamp: UnixTimestamp, msg: Vec<u8>, sender: Pubkey) -> Self {
        Self {
            tag: Tag::Message,
            kind,
            timestamp,
            msg,
            sender,
        }
    }

    pub fn find_from_keys(
        index: u32,
        from_key: &Pubkey,
        to_key: &Pubkey,
        program_id: &Pubkey,
    ) -> (Pubkey, u8) {
        let i = index.to_string();
        let (key_1, key_2) = order_keys(from_key, to_key);
        let (message_key, bump) = Pubkey::find_program_address(
            &[
                Message::SEED.as_bytes(),
                i.as_bytes(),
                &key_1.to_bytes(),
                &key_2.to_bytes(),
            ],
            program_id,
        );
        return (message_key, bump);
    }

    pub fn create_from_keys(
        index: u32,
        from_key: &Pubkey,
        to_key: &Pubkey,
        program_id: &Pubkey,
        bump: u8,
    ) -> Pubkey {
        let (key_1, key_2) = order_keys(from_key, to_key);
        let i = index.to_string();
        let seeds = &[
            Message::SEED.as_bytes(),
            i.as_bytes(),
            &key_1.to_bytes(),
            &key_2.to_bytes(),
            &[bump],
        ];
        Pubkey::create_program_address(seeds, program_id).unwrap()
    }

    pub fn save(&self, mut dst: &mut [u8]) {
        self.serialize(&mut dst).unwrap()
    }

    pub fn from_account_info(a: &AccountInfo) -> Result<Message, ProgramError> {
        let mut data = &a.data.borrow() as &[u8];
        if data[0] != Tag::Message as u8 && data[0] != Tag::Uninitialized as u8 {
            return Err(JabberError::DataTypeMismatch.into());
        }
        let result = Message::deserialize(&mut data)?;
        Ok(result)
    }
}
