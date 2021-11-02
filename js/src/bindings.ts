import {
  CreateProfile,
  JABBER_ID,
  CreateThread,
  SetUserProfile,
  SendMessage,
  CreateGroupThread,
  EditGroupThread,
  AddGroupAdmin,
  RemoveGroupAdmin,
} from "./instructions";
import { Connection, PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import { Profile, Thread, MessageType, Message, GroupThread } from "./state";

/**
 *
 * @param profileOwner Owner of the profile
 * @param name Name to display on the profile
 * @param bio Bio to display on the profile
 * @param lamportsPerMessage Amount of lamports the user wants to receive (i.e be paid) per message
 * @returns
 */
export const createProfile = async (
  profileOwner: PublicKey,
  name: string,
  bio: string,
  lamportsPerMessage: number
) => {
  const [profile] = await PublicKey.findProgramAddress(
    Profile.generateSeeds(profileOwner),
    JABBER_ID
  );
  const instruction = new CreateProfile({
    name: name,
    bio: bio,
    lamportsPerMessage: new BN(lamportsPerMessage),
  }).getInstruction(profile, profileOwner, profileOwner);

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

  const instruction = new CreateThread({
    sender: sender.toBuffer(),
    receiver: receiver.toBuffer(),
  }).getInstruction(thread, feePayer);

  return instruction;
};

/**
 *
 * @param profileOwner Owner of the profile
 * @param name Name to display on the profile
 * @param bio Bio to display on the profile
 * @param lamportsPerMessage Amount of lamports the user wants to receive (i.e be paid) per message
 * @returns
 */
export const setUserProfile = async (
  profileOwner: PublicKey,
  name: string,
  bio: string,
  lamportsPerMessage: number
) => {
  const [profile] = await PublicKey.findProgramAddress(
    Profile.generateSeeds(profileOwner),
    JABBER_ID
  );

  const instruction = new SetUserProfile({
    name: name,
    bio: bio,
    lamportsPerMessage: new BN(lamportsPerMessage),
  }).getInstruction(profileOwner, profile);

  return instruction;
};

/**
 *
 * @param connection The solana connection object to the RPC node
 * @param sender The sender of the message
 * @param receiver The receiver of the message
 * @param message The message as a Uint8Array
 * @param kind Type of the message
 * @returns
 */
export const sendMessage = async (
  connection: Connection,
  sender: PublicKey,
  receiver: PublicKey,
  message: Uint8Array,
  kind: MessageType
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

  const instruction = new SendMessage({
    kind: kind,
    message: message,
  }).getInstruction(
    sender,
    receiver,
    threadAccount,
    receiverProfile,
    messageAccount
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
 * @returns
 */
export const createGroupThread = async (
  groupName: string,
  destinationWallet: PublicKey,
  lamportsPerMessage: BN,
  admins: PublicKey[],
  owner: PublicKey,
  mediaEnabled: boolean,
  feePayer: PublicKey
) => {
  const groupThread = await GroupThread.getKey(groupName, owner);

  const instruction = new CreateGroupThread({
    groupName,
    destinationWallet: destinationWallet.toBuffer(),
    lamportsPerMessage,
    admins: admins.map((e) => e.toBuffer()),
    owner: owner.toBuffer(),
    mediaEnabled,
  }).getInstruction(groupThread, feePayer);

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
  mediaEnabled: boolean
) => {
  const groupThread = await GroupThread.getKey(groupName, owner);

  const instruction = new EditGroupThread({
    destinationWallet: destinationWallet.toBuffer(),
    lamportsPerMessage,
    owner: owner.toBuffer(),
    mediaEnabled: mediaEnabled,
  }).getInstruction(owner, groupThread);

  return instruction;
};

/**
 *
 * @param groupKey Address of the group thread
 * @param adminToAdd Address of the admin to add
 * @param groupOwner Owner of the group
 * @returns
 */
export const addAdminToGroup = async (
  groupKey: PublicKey,
  adminToAdd: PublicKey,
  groupOwner: PublicKey
) => {
  const instruction = new AddGroupAdmin({
    adminAddress: adminToAdd.toBuffer(),
  }).getInstruction(groupKey, groupOwner);

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
export const removeAdminFromGroup = async (
  groupKey: PublicKey,
  adminToRemove: PublicKey,
  adminIndex: number,
  groupOwner: PublicKey
) => {
  const instruction = new RemoveGroupAdmin({
    adminAddress: adminToRemove.toBuffer(),
    adminIndex: adminIndex,
  }).getInstruction(groupKey, groupOwner);

  return instruction;
};
