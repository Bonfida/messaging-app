import {
  CreateProfile,
  JABBER_ID,
  CreateThread,
  SetUserProfile,
  SendMessage,
} from "./instructions";
import { Connection, PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import { Profile, Thread, MessageType, Message } from "./state";

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
