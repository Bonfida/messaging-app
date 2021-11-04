use crate::{error::JabberError, utils::order_keys};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, clock::UnixTimestamp, entrypoint::ProgramResult,
    program_error::ProgramError, pubkey::Pubkey,
};

pub const MAX_NAME_LENGTH: usize = 100;
pub const MAX_BIO_LENGTH: usize = 100;
pub const MAX_MSG_LEN: usize = 1_000;

pub const MAX_PROFILE_LEN: usize = 1 + MAX_NAME_LENGTH + MAX_BIO_LENGTH + 8 + 1;

pub const MAX_THREAD_LEN: usize = 1 + 4 + 32 + 32 + 1;

pub const MAX_GROUP_NAME_LEN: usize = 100;
pub const MAX_ADMIN_LEN: usize = 10;
pub const MAX_HASH_LEN: usize = 64;

pub const MAX_GROUP_THREAD_LEN: usize = 1 // tag
    + (4 + MAX_GROUP_NAME_LEN) // group_name
    + 4 // msg_count
    + 32 // destination_wallet
    + 8 // lamports_per_message
    + 1 // bump
    + (4 + MAX_ADMIN_LEN * 32) // admins
    + 32 // owner
    + 1 // media_enabled
    + (4 + MAX_HASH_LEN) // group_pic_hash
    + 1; // admin_only

pub const MAX_GROUP_THREAD_INDEX: usize = 1 + 4 + MAX_GROUP_NAME_LEN + 32 + 32;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Copy)]
pub enum Tag {
    Uninitialized,
    Profile,
    Thread,
    Message,
    Jabber,
    GroupThread,
    GroupThreadIndex,
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
        (user_profile_key, bump)
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
        (thread_key, bump)
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
    UnencryptedImage,
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
        (message_key, bump)
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

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct GroupThread {
    pub tag: Tag,
    pub group_name: String,
    pub msg_count: u32,
    pub destination_wallet: Pubkey,
    pub lamports_per_message: u64,
    pub bump: u8,
    // Admins of the group (fee exempt)
    pub admins: Vec<Pubkey>,
    // Owner of the group (fee exempt)
    pub owner: Pubkey,
    // Whether users can post media (images, videos and audios)
    pub media_enabled: bool,
    // IPFS hash of the group
    pub group_pic_hash: Option<String>,
    // Whether admins only can post messages
    pub admin_only: bool,
}

impl GroupThread {
    pub const SEED: &'static str = "group_thread";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        group_name: String,
        destination_wallet: Pubkey,
        lamports_per_message: u64,
        bump: u8,
        admins: Vec<Pubkey>,
        owner: Pubkey,
        media_enabled: bool,
        admin_only: bool,
    ) -> Self {
        Self {
            tag: Tag::GroupThread,
            group_name,
            msg_count: 0,
            destination_wallet,
            lamports_per_message,
            bump,
            admins,
            owner,
            media_enabled,
            group_pic_hash: None,
            admin_only,
        }
    }

    pub fn create_from_destination_wallet_and_name(
        group_name: String,
        owner: Pubkey,
        program_id: &Pubkey,
        bump: u8,
    ) -> Pubkey {
        let seeds = &[
            GroupThread::SEED.as_bytes(),
            group_name.as_bytes(),
            &owner.to_bytes(),
            &[bump],
        ];
        Pubkey::create_program_address(seeds, program_id).unwrap()
    }

    pub fn find_from_destination_wallet_and_name(
        group_name: String,
        owner: Pubkey,
        program_id: &Pubkey,
    ) -> (Pubkey, u8) {
        let seeds = &[
            GroupThread::SEED.as_bytes(),
            group_name.as_bytes(),
            &owner.to_bytes(),
        ];
        let (ama_thread_key, bump) = Pubkey::find_program_address(seeds, program_id);
        (ama_thread_key, bump)
    }

    pub fn save(&self, mut dst: &mut [u8]) {
        self.serialize(&mut dst).unwrap()
    }

    pub fn increment_msg_count(&mut self) {
        self.msg_count += 1;
    }

    pub fn from_account_info(a: &AccountInfo) -> Result<GroupThread, ProgramError> {
        let mut data = &a.data.borrow() as &[u8];
        if data[0] != Tag::GroupThread as u8 && data[0] != Tag::Uninitialized as u8 {
            return Err(JabberError::DataTypeMismatch.into());
        }
        let result = GroupThread::deserialize(&mut data)?;
        Ok(result)
    }

    pub fn is_fee_exempt(&self, sender: Pubkey, admin_index: Option<usize>) -> bool {
        if self.destination_wallet == sender {
            return true;
        }
        if let Some(idx) = admin_index {
            return self.admins[idx] == sender;
        }
        false
    }

    pub fn add_admin(&mut self, admin_address: Pubkey) -> ProgramResult {
        if self.admins.len() > MAX_ADMIN_LEN {
            return Err(JabberError::MaxAdminsReached.into());
        }
        self.admins.push(admin_address);
        Ok(())
    }

    pub fn remove_admin(&mut self, admin_address: Pubkey, admin_index: usize) -> ProgramResult {
        let deleted = self.admins.remove(admin_index);
        if admin_address != deleted {
            return Err(JabberError::InvalidAdminIndex.into());
        }
        Ok(())
    }
}

// To keep track of users' groups
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct GroupThreadIndex {
    pub tag: Tag,
    pub group_name: String,
    pub group_thread_key: Pubkey,
    pub owner: Pubkey,
}

impl GroupThreadIndex {
    pub const SEED: &'static str = "group_thread_index";

    pub fn new(group_name: String, group_thread_key: Pubkey, owner: Pubkey) -> Self {
        Self {
            tag: Tag::GroupThreadIndex,
            group_name,
            group_thread_key,
            owner,
        }
    }

    pub fn create_address(
        group_name: String,
        group_thread_key: Pubkey,
        owner: Pubkey,
        program_id: &Pubkey,
        bump: u8,
    ) -> Pubkey {
        let seeds = &[
            GroupThreadIndex::SEED.as_bytes(),
            group_name.as_bytes(),
            &owner.to_bytes(),
            &group_thread_key.to_bytes(),
            &[bump],
        ];
        Pubkey::create_program_address(seeds, program_id).unwrap()
    }

    pub fn find_address(
        group_name: String,
        group_thread_key: Pubkey,
        owner: Pubkey,
        program_id: &Pubkey,
    ) -> (Pubkey, u8) {
        let seeds = &[
            GroupThreadIndex::SEED.as_bytes(),
            group_name.as_bytes(),
            &owner.to_bytes(),
            &group_thread_key.to_bytes(),
        ];
        let (ama_thread_key, bump) = Pubkey::find_program_address(seeds, program_id);
        (ama_thread_key, bump)
    }

    pub fn save(&self, mut dst: &mut [u8]) {
        self.serialize(&mut dst).unwrap()
    }

    pub fn from_account_info(a: &AccountInfo) -> Result<GroupThreadIndex, ProgramError> {
        let mut data = &a.data.borrow() as &[u8];
        if data[0] != Tag::GroupThreadIndex as u8 && data[0] != Tag::Uninitialized as u8 {
            return Err(JabberError::DataTypeMismatch.into());
        }
        let result = GroupThreadIndex::deserialize(&mut data)?;
        Ok(result)
    }
}
