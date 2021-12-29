use crate::{error::JabberError, utils::order_keys};
use bonfida_utils::BorshSize;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, clock::UnixTimestamp, entrypoint::ProgramResult,
    program_error::ProgramError, pubkey::Pubkey,
};

pub const MAX_NAME_LENGTH: usize = 100;
pub const MAX_BIO_LENGTH: usize = 100;
pub const MAX_GROUP_NAME_LEN: usize = 100;
pub const MAX_ADMIN_LEN: usize = 10;
pub const MAX_HASH_LEN: usize = 64;

pub const MAX_PROFILE_LEN: usize =
    1 + MAX_HASH_LEN + MAX_NAME_LENGTH + MAX_BIO_LENGTH + 8 + 1 + 4 + 4;

pub const MAX_GROUP_THREAD_LEN: usize = 1 // tag
    + 1 // visible
    + 32 // owner
    + 8 // last message time
    + 32 // destination_wallet
    + 4 // msg_count
    + 8 // lamports_per_message
    + 1 // bump
    + 1 // media_enabled
    + 1 // admin_only
    + (4 + MAX_HASH_LEN) // group_pic_hash
    + (4 + MAX_GROUP_NAME_LEN) // group_name
    + (4 + MAX_ADMIN_LEN * 32); // admins

pub const MAX_GROUP_THREAD_INDEX: usize = 1 + 4 + MAX_GROUP_NAME_LEN + 32 + 32;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Copy, BorshSize)]
pub enum Tag {
    Uninitialized,
    Profile,
    Thread,
    Message,
    Jabber,
    GroupThread,
    GroupThreadIndex,
    Subscription,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Profile {
    pub tag: Tag,
    pub picture_hash: String,
    pub display_domain_name: String,
    pub bio: String,
    pub lamports_per_message: u64,
    pub bump: u8,
    pub tips_sent: u32,
    pub tips_received: u32,
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

    pub fn new(
        picture_hash: String,
        display_domain_name: String,
        bio: String,
        lamports_per_message: u64,
        bump: u8,
    ) -> Self {
        Self {
            tag: Tag::Profile,
            display_domain_name,
            picture_hash,
            bio,
            lamports_per_message,
            bump,
            tips_sent: 0,
            tips_received: 0,
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

#[derive(BorshSerialize, BorshDeserialize, BorshSize)]
pub struct Thread {
    pub tag: Tag,
    pub msg_count: u32,
    pub user_1: Pubkey,
    pub user_2: Pubkey,
    pub last_message_time: UnixTimestamp,
    pub bump: u8,
}

impl Thread {
    pub const SEED: &'static str = "thread";

    pub fn new(user_1: Pubkey, user_2: Pubkey, bump: u8, last_message_time: UnixTimestamp) -> Self {
        Self {
            tag: Tag::Thread,
            msg_count: 0,
            user_1,
            user_2,
            bump,
            last_message_time,
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

    pub fn increment_msg_count(&mut self, current_time: i64) {
        self.last_message_time = current_time;
        self.msg_count += 1;
    }
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, BorshSize)]
pub enum MessageType {
    EncryptedText,
    UnencryptedText,
    EncryptedMedia,
    UnencryptedMedia,
    Deleted,
}

#[derive(BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Message {
    pub tag: Tag,
    // Message type
    pub kind: MessageType,
    // Time at which the message was sent
    pub timestamp: UnixTimestamp,
    // Sender of the message
    pub sender: Pubkey,
    // If the message is a response to another message
    pub replies_to: Pubkey,
    // Likes counter
    pub likes_count: u16,
    // Dislikes counter
    pub dislikes_count: u16,
    // Message sent
    pub msg: Vec<u8>,
}

impl Message {
    pub const SEED: &'static str = "message";

    pub fn new(
        kind: MessageType,
        timestamp: UnixTimestamp,
        msg: Vec<u8>,
        sender: Pubkey,
        replies_to: Pubkey,
    ) -> Self {
        Self {
            tag: Tag::Message,
            kind,
            timestamp,
            msg,
            sender,
            replies_to,
            likes_count: 0,
            dislikes_count: 0,
        }
    }

    pub fn find_key(
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

    pub fn create_key(
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

#[derive(BorshSerialize, BorshDeserialize)]
pub struct GroupThread {
    pub tag: Tag,
    // Whether to suggest the group in the app
    pub visible: bool,
    // Owner of the group (fee exempt)
    pub owner: Pubkey,
    // Time at which the message was sent
    pub last_message_time: UnixTimestamp,
    // Destination of the fees
    pub destination_wallet: Pubkey,
    // Message counter for PDA derivation
    pub msg_count: u32,
    // Fee per message
    pub lamports_per_message: u64,
    // PDA derivation bump
    pub bump: u8,
    // Whether users can post media (images, videos and audios)
    pub media_enabled: bool,
    // Whether admins only can post messages
    pub admin_only: bool,
    // IPFS hash of the group
    pub group_pic_hash: Option<String>,
    // Human readable group name
    pub group_name: String,
    // Admins of the group (fee exempt)
    pub admins: Vec<Pubkey>,
}

impl GroupThread {
    pub const SEED: &'static str = "group_thread";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        visible: bool,
        group_name: String,
        destination_wallet: Pubkey,
        lamports_per_message: u64,
        bump: u8,
        admins: Vec<Pubkey>,
        owner: Pubkey,
        media_enabled: bool,
        admin_only: bool,
        current_time: i64,
    ) -> Self {
        Self {
            tag: Tag::GroupThread,
            visible,
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
            last_message_time: current_time,
        }
    }

    pub fn create_key(group_name: String, owner: Pubkey, program_id: &Pubkey, bump: u8) -> Pubkey {
        let seeds = &[
            GroupThread::SEED.as_bytes(),
            group_name.as_bytes(),
            &owner.to_bytes(),
            &[bump],
        ];
        Pubkey::create_program_address(seeds, program_id).unwrap()
    }

    pub fn find_key(group_name: String, owner: Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
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

    pub fn increment_msg_count(&mut self, current_time: i64) {
        self.last_message_time = current_time;
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

    pub fn is_fee_exempt(&self, sender: Pubkey, admin_index: Option<u64>) -> bool {
        if self.destination_wallet == sender {
            return true;
        }
        if let Some(idx) = admin_index {
            return self.admins[idx as usize] == sender;
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
#[derive(BorshSerialize, BorshDeserialize, BorshSize)]
pub struct GroupThreadIndex {
    pub tag: Tag,
    // Group thread of the index
    pub group_thread_key: Pubkey,
    // Owner of the index
    pub owner: Pubkey,
    // Name of the group
    pub group_name: String,
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

    pub fn create_key(
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

    pub fn find_key(
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

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Subscription {
    // Pubkey of the subscriber
    pub subscriber: Pubkey,
    // Pubkey of the person the subscriber subscribed to
    pub subscribed_to: Pubkey,
}

impl Subscription {
    pub const SEED: &'static str = "subscription";

    pub fn new(subscriber: Pubkey, subscribed_to: Pubkey) -> Self {
        Self {
            subscriber,
            subscribed_to,
        }
    }

    pub fn create_key(
        subscriber: &Pubkey,
        subscribed_to: &Pubkey,
        program_id: &Pubkey,
        bump: u8,
    ) -> Pubkey {
        let seeds = &[
            Subscription::SEED.as_bytes(),
            &subscriber.to_bytes(),
            &subscribed_to.to_bytes(),
            &[bump],
        ];
        Pubkey::create_program_address(seeds, program_id).unwrap()
    }

    pub fn find_key(
        subscriber: &Pubkey,
        subscribed_to: &Pubkey,
        program_id: &Pubkey,
    ) -> (Pubkey, u8) {
        let seeds = &[
            Subscription::SEED.as_bytes(),
            &subscriber.to_bytes(),
            &subscribed_to.to_bytes(),
        ];
        Pubkey::find_program_address(seeds, program_id)
    }

    pub fn save(&self, mut dst: &mut [u8]) {
        self.serialize(&mut dst).unwrap()
    }

    pub fn from_account_info(a: &AccountInfo) -> Result<Subscription, ProgramError> {
        let mut data = &a.data.borrow() as &[u8];
        if data[0] != Tag::Subscription as u8 && data[0] != Tag::Uninitialized as u8 {
            return Err(JabberError::DataTypeMismatch.into());
        }
        let result = Subscription::deserialize(&mut data)?;
        Ok(result)
    }
}
