import {
  createProfileInstruction,
  createThreadInstruction,
  setUserProfileInstruction,
  sendMessageInstruction,
  createGroupThreadInstruction,
  editGroupThreadInstruction,
  addAdminToGroupInstruction,
  removeAdminFromGroupInstruction,
  createGroupIndexInstruction,
  sendMessageGroupInstruction,
  deleteMessageInstruction,
  deleteGroupMessageInstruction,
} from "./raw_instructions";
import {
  Connection,
  MemcmpFilter,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import BN from "bn.js";
import {
  Profile,
  Thread,
  MessageType,
  Message,
  GroupThread,
  GroupThreadIndex,
} from "./state";

export const JABBER_ID = PublicKey.default;
export const SOL_VAULT = new PublicKey(
  "GcWEQ9K78FV7LEHteFVciYApERk5YvQuFDQPk1yYJVXi"
);

/**
 *
 * @param profileOwner Owner of the profile
 * @param displayDomainName Domain name to display on the profile
 * @param pictureHash The IPFS hash of the profile pic
 * @param bio Bio to display on the profile
 * @param lamportsPerMessage Amount of lamports the user wants to receive (i.e be paid) per message
 * @returns
 */
export const createProfile = async (
  profileOwner: PublicKey,
  displayDomainName: string,
  pictureHash: string,
  bio: string,
  lamportsPerMessage: number
) => {
  const [profile] = await PublicKey.findProgramAddress(
    Profile.generateSeeds(profileOwner),
    JABBER_ID
  );
  const instruction = new createProfileInstruction({
    displayDomainName,
    bio,
    pictureHash,
    lamportsPerMessage: new BN(lamportsPerMessage),
  }).getInstruction(
    JABBER_ID,
    SystemProgram.programId,
    profile,
    profileOwner,
    profileOwner
  );

  return instruction;
};

/**
 *
 * @param sender User 1 of the thread
 * @param receiver User 2 of the thread
 * @param feePayer Fee payer of the instruction
 * @returns
 */
export const createThread = async (
  sender: PublicKey,
  receiver: PublicKey,
  feePayer: PublicKey
) => {
  const [thread] = await PublicKey.findProgramAddress(
    Thread.generateSeeds(sender, receiver),
    JABBER_ID
  );

  const instruction = new createThreadInstruction({
    senderKey: sender.toBuffer(),
    receiverKey: receiver.toBuffer(),
  }).getInstruction(JABBER_ID, SystemProgram.programId, thread, feePayer);

  return instruction;
};

/**
 *
 * @param pictureHash IPFS hash of the profile pic
 * @param displayDomainName Display domain name
 * @param bio User bio
 * @param lamportsPerMessage lamports per message
 * @param allowDm If the user allows DM
 * @param profileOwner Profile owner
 * @returns
 */
export const setUserProfile = async (
  pictureHash: string,
  displayDomainName: string,
  bio: string,
  lamportsPerMessage: number,
  allowDm: boolean,
  profileOwner: PublicKey
) => {
  const [profile] = await PublicKey.findProgramAddress(
    Profile.generateSeeds(profileOwner),
    JABBER_ID
  );

  const instruction = new setUserProfileInstruction({
    pictureHash,
    displayDomainName,
    bio,
    lamportsPerMessage: new BN(lamportsPerMessage),
    allowDm: allowDm ? 1 : 0,
  }).getInstruction(JABBER_ID, profileOwner, profile);

  return instruction;
};

/**
 *
 * @param connection The RPC connection object
 * @param sender The message sender account
 * @param receiver The message receiver account
 * @param message The message
 * @param kind The message kind
 * @param repliesTo If the message is a replie to another message (if not PublicKey.default())
 * @returns
 */
export const sendMessage = async (
  connection: Connection,
  sender: PublicKey,
  receiver: PublicKey,
  message: Uint8Array,
  kind: MessageType,
  repliesTo: PublicKey
) => {
  const [receiverProfile] = await PublicKey.findProgramAddress(
    Profile.generateSeeds(receiver),
    JABBER_ID
  );
  const [threadAccount] = await PublicKey.findProgramAddress(
    Thread.generateSeeds(sender, receiver),
    JABBER_ID
  );

  const thread = await Thread.retrieve(connection, sender, receiver);

  const [messageAccount] = await PublicKey.findProgramAddress(
    Message.generateSeeds(thread.msgCount, sender, receiver),
    JABBER_ID
  );

  const instruction = new sendMessageInstruction({
    kind: kind,
    message: Array.from(message),
    repliesTo: repliesTo.toBuffer(),
  }).getInstruction(
    JABBER_ID,
    SystemProgram.programId,
    sender,
    receiver,
    threadAccount,
    receiverProfile,
    messageAccount,
    SOL_VAULT
  );

  return instruction;
};

/**
 *
 * @param connection The solana connection object to the RPC node
 * @param user The user to fetch threads for
 * @returns
 */
export const retrieveUserThread = async (
  connection: Connection,
  user: PublicKey
) => {
  let filters_1 = [
    {
      memcmp: {
        offset: 1 + 4,
        bytes: user.toBase58(),
      },
    },
  ];
  const filters_2 = [
    {
      memcmp: {
        offset: 1 + 4 + 32,
        bytes: user.toBase58(),
      },
    },
  ];
  const result_1 = await connection.getProgramAccounts(JABBER_ID, {
    filters: filters_1,
  });
  const result_2 = await connection.getProgramAccounts(JABBER_ID, {
    filters: filters_2,
  });
  return result_1.concat(result_2);
};

/**
 *
 * @param groupName Name of the group
 * @param destinationWallet Wallet that will receive the fees
 * @param lamportsPerMessage SOL fee per message
 * @param admins Admins of the group
 * @param owner Owner of the group (only address that will be able to edit the group)
 * @param mediaEnabled Is it possible to send media (images, videos and audios)?
 * @param feePayer Fee payer of the instruction
 * @param visible If the group can be visible for others to join. Only used for the app, at the end of the day everything is visible on-chain
 * @returns
 */
export const createGroupThread = async (
  groupName: string,
  destinationWallet: PublicKey,
  lamportsPerMessage: BN,
  admins: PublicKey[],
  owner: PublicKey,
  mediaEnabled: boolean,
  adminOnly: boolean,
  feePayer: PublicKey,
  visible: boolean
) => {
  const groupThread = await GroupThread.getKey(groupName, owner);

  const instruction = new createGroupThreadInstruction({
    groupName,
    destinationWallet: destinationWallet.toBuffer(),
    lamportsPerMessage,
    admins: admins.map((e) => e.toBuffer()),
    owner: owner.toBuffer(),
    mediaEnabled: mediaEnabled ? 1 : 0,
    adminOnly: adminOnly ? 1 : 0,
    visible: visible ? 1 : 0,
  }).getInstruction(JABBER_ID, SystemProgram.programId, groupThread, feePayer);

  return instruction;
};

/**
 *
 * @param groupName Name of the group
 * @param owner Owner of the group
 * @param destinationWallet allet that will receive the fees
 * @param lamportsPerMessage SOL fee per message
 * @param mediaEnabled Is it possible to send media (images, videos and audios)?
 * @returns
 */
export const editGroupThread = async (
  groupName: string,
  owner: PublicKey,
  destinationWallet: PublicKey,
  lamportsPerMessage: BN,
  mediaEnabled: boolean,
  adminOnly: boolean,
  groupPicHash: string,
  visible: boolean
) => {
  const groupThread = await GroupThread.getKey(groupName, owner);

  const instruction = new editGroupThreadInstruction({
    destinationWallet: destinationWallet.toBuffer(),
    lamportsPerMessage,
    owner: owner.toBuffer(),
    mediaEnabled: mediaEnabled ? 1 : 0,
    adminOnly: adminOnly ? 1 : 0,
    groupPicHash,
    visible: visible ? 1 : 0,
  }).getInstruction(JABBER_ID, owner, groupThread);

  return instruction;
};

/**
 *
 * @param groupKey Address of the group thread
 * @param adminToAdd Address of the admin to add
 * @param groupOwner Owner of the group
 * @returns
 */
export const addAdminToGroup = (
  groupKey: PublicKey,
  adminToAdd: PublicKey,
  groupOwner: PublicKey
) => {
  const instruction = new addAdminToGroupInstruction({
    adminAddress: adminToAdd.toBuffer(),
  }).getInstruction(JABBER_ID, groupKey, groupOwner);

  return instruction;
};

/**
 *
 * @param groupKey Address of the group thread
 * @param adminToRemove Address of the admin to remove
 * @param adminIndex Index of the admin in the Vec<Pubkey> of admins (cf GroupThread state)
 * @param groupOwner Owner of the group
 * @returns
 */
export const removeAdminFromGroup = (
  groupKey: PublicKey,
  adminToRemove: PublicKey,
  adminIndex: number,
  groupOwner: PublicKey
) => {
  const instruction = new removeAdminFromGroupInstruction({
    adminAddress: adminToRemove.toBuffer(),
    adminIndex: new BN(adminIndex),
  }).getInstruction(JABBER_ID, groupKey, groupOwner);

  return instruction;
};

export const createGroupIndex = async (
  groupName: string,
  owner: PublicKey,
  groupThread: PublicKey
) => {
  const groupIndex = await GroupThreadIndex.getKey(
    groupName,
    owner,
    groupThread
  );
  const instruction = new createGroupIndexInstruction({
    groupName,
    groupThreadKey: groupThread.toBuffer(),
    owner: owner.toBuffer(),
  }).getInstruction(JABBER_ID, SystemProgram.programId, groupIndex, owner);

  return instruction;
};

/**
 *
 * @param kind Message type
 * @param message Message to send
 * @param groupName Name of the group
 * @param sender User sending the message
 * @param groupThread Key of the group thread
 * @param destinationWallet Destination wallet of the group
 * @param messageAccount Account of the message
 * @param adminIndex Admin index
 */
export const sendMessageGroup = async (
  kind: MessageType,
  message: Uint8Array,
  groupName: string,
  sender: PublicKey,
  groupThread: PublicKey,
  destinationWallet: PublicKey,
  messageAccount: PublicKey,
  adminIndex: number,
  repliesTo?: PublicKey
) => {
  const instruction = new sendMessageGroupInstruction({
    kind: kind as number,
    message: Array.from(message),
    groupName,
    adminIndex,
    repliesTo: repliesTo ? repliesTo.toBuffer() : PublicKey.default.toBuffer(),
  }).getInstruction(
    JABBER_ID,
    SystemProgram.programId,
    sender,
    groupThread,
    destinationWallet,
    messageAccount,
    SOL_VAULT
  );

  return instruction;
};

/**
 *
 * @param connection The solana connection object to the RPC node
 * @param user The user to fetch the groups for
 * @returns
 */
export const retrieveUserGroups = async (
  connection: Connection,
  user: PublicKey
) => {
  let filters: MemcmpFilter[] = [
    {
      memcmp: {
        offset: 1 + 32,
        bytes: user.toBase58(),
      },
    },
    {
      memcmp: {
        offset: 0,
        bytes: "7",
      },
    },
  ];
  const result = await connection.getProgramAccounts(JABBER_ID, { filters });

  return result;
};

/**
 *
 * @param sender Original sender of the message
 * @param receiver Original receiver of the message
 * @param message Account of the message to delete
 * @param messageIndex Index of the message in the thread
 * @returns
 */
export const deleteMessage = async (
  sender: PublicKey,
  receiver: PublicKey,
  message: PublicKey,
  messageIndex: number
) => {
  const instruction = new deleteMessageInstruction({
    messageIndex,
  }).getInstruction(JABBER_ID, sender, receiver, message);

  return instruction;
};

/**
 *
 * @param groupThread Group thread address
 * @param message Account of the message to delete
 * @param feePayer Fee payer (either owner, admin or original sender)
 * @param messageIndex Index of the message in the thread
 * @param owner Owner of the group
 * @param groupName Name of the group
 * @param adminIndex The index of the admin in the list of admins (if feePayer is an admin) | undefined
 * @returns
 */
export const deleteGroupMessage = async (
  groupThread: PublicKey,
  message: PublicKey,
  feePayer: PublicKey,
  messageIndex: number,
  owner: PublicKey,
  groupName: string,
  adminIndex: number
) => {
  const instruction = new deleteGroupMessageInstruction({
    messageIndex,
    owner: owner.toBuffer(),
    adminIndex: adminIndex,
    groupName,
  }).getInstruction(JABBER_ID, groupThread, message, feePayer);

  return instruction;
};

export const retrieveGroupMembers = async (
  connection: Connection,
  group: PublicKey
) => {
  let filters: MemcmpFilter[] = [
    {
      memcmp: {
        offset: 1,
        bytes: group.toBase58(),
      },
    },
    {
      memcmp: {
        offset: 0,
        bytes: "7",
      },
    },
  ];
  const result = await connection.getProgramAccounts(JABBER_ID, { filters });

  return result.map(
    (acc) => GroupThreadIndex.deserialize(acc.account.data).owner
  );
};
