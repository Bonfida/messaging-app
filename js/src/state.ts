import { PublicKey, Connection } from "@solana/web3.js";
import BN from "bn.js";
import { Schema, deserializeUnchecked, deserialize } from "borsh";
import { JABBER_ID } from "./instructions";
import { orderKeys } from "./utils";

export enum Tag {
  Uninitialized = 0,
  Profile = 1,
  Thread = 2,
  Message = 3,
  Jabber = 4,
  GroupThread = 5,
}

export enum MessageType {
  Encrypted = 0,
  Unencrypted = 1,
  EncryptedImage = 2,
  UnencryptedImage = 3,
}

export class Profile {
  tag: Tag;
  name: string;
  bio: string;
  lamportsPerMessage: BN;
  bump: number;

  static schema: Schema = new Map([
    [
      Profile,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["name", "string"],
          ["bio", "string"],
          ["lamportsPerMessage", "u64"],
          ["bump", "u8"],
        ],
      },
    ],
  ]);

  constructor(obj: {
    name: string;
    bio: string;
    lamportsPerMessage: BN;
    bump: number;
  }) {
    this.tag = Tag.Profile;
    this.name = obj.name;
    this.bio = obj.bio;
    this.lamportsPerMessage = obj.lamportsPerMessage;
    this.bump = obj.bump;
  }

  static deserialize(data: Buffer) {
    return deserializeUnchecked(this.schema, Profile, data);
  }

  static async retrieve(connection: Connection, owner: PublicKey) {
    const [profile] = await PublicKey.findProgramAddress(
      Profile.generateSeeds(owner),
      JABBER_ID
    );

    const accountInfo = await connection.getAccountInfo(profile);

    if (!accountInfo?.data) {
      throw new Error("No profile found");
    }

    return this.deserialize(accountInfo?.data);
  }

  static generateSeeds(profileOwner: PublicKey) {
    return [Buffer.from("profile"), profileOwner.toBuffer()];
  }
}

export class Thread {
  tag: Tag;
  msgCount: number;
  user1: PublicKey;
  user2: PublicKey;
  bump: number;

  static schema: Schema = new Map([
    [
      Thread,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["msgCount", "u32"],
          ["user1", [32]],
          ["user2", [32]],
          ["bump", "u8"],
        ],
      },
    ],
  ]);

  constructor(obj: {
    msgCount: number;
    user1: Uint8Array;
    user2: Uint8Array;
    bump: number;
  }) {
    this.tag = Tag.Thread;
    this.msgCount = obj.msgCount;
    this.user1 = new PublicKey(obj.user1);
    this.user2 = new PublicKey(obj.user2);
    this.bump = obj.bump;
  }

  static deserialize(data: Buffer) {
    return deserialize(this.schema, Thread, data);
  }

  static generateSeeds(sender: PublicKey, receiver: PublicKey) {
    const [key1, key2] = orderKeys(sender, receiver);
    return [Buffer.from("thread"), key1.toBuffer(), key2.toBuffer()];
  }

  static async getKeys(sender: PublicKey, receiver: PublicKey) {
    const [thread] = await PublicKey.findProgramAddress(
      Thread.generateSeeds(sender, receiver),
      JABBER_ID
    );
    return thread;
  }

  static async retrieve(
    connection: Connection,
    sender: PublicKey,
    receiver: PublicKey
  ) {
    const [thread] = await PublicKey.findProgramAddress(
      Thread.generateSeeds(sender, receiver),
      JABBER_ID
    );
    const accountInfo = await connection.getAccountInfo(thread);

    if (!accountInfo?.data) {
      throw new Error("Thread not found");
    }

    return this.deserialize(accountInfo.data);
  }

  static async retrieveFromKey(connection: Connection, key: PublicKey) {
    const accountInfo = await connection.getAccountInfo(key);

    if (!accountInfo?.data) {
      throw new Error("Thread not found");
    }

    return this.deserialize(accountInfo.data);
  }
}

export class Message {
  tag: Tag;
  kind: MessageType;
  timestamp: BN;
  msg: Uint8Array;
  sender: PublicKey;

  static schema: Schema = new Map([
    [
      Message,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["kind", "u8"],
          ["timestamp", "u64"],
          ["msg", ["u8"]],
          ["sender", [32]],
        ],
      },
    ],
  ]);

  constructor(obj: {
    kind: MessageType;
    timestamp: BN;
    msg: Uint8Array;
    sender: Uint8Array;
  }) {
    this.tag = Tag.Message;
    this.kind = obj.kind;
    this.timestamp = obj.timestamp;
    this.msg = obj.msg;
    this.sender = new PublicKey(obj.sender);
  }

  static deserialize(data: Buffer) {
    return deserializeUnchecked(this.schema, Message, data);
  }

  static generateSeeds(
    messageCount: number,
    sender: PublicKey,
    receiver: PublicKey
  ) {
    const [key1, key2] = orderKeys(sender, receiver);
    return [
      Buffer.from("message"),
      Buffer.from(messageCount.toString()),
      key1.toBuffer(),
      key2.toBuffer(),
    ];
  }

  static async retrieveFromIndex(
    connection: Connection,
    index: number,
    receiver: PublicKey,
    sender: PublicKey
  ) {
    const [messageAccount] = await PublicKey.findProgramAddress(
      this.generateSeeds(index, sender, receiver),
      JABBER_ID
    );
    const accountInfo = await connection.getAccountInfo(messageAccount);
    if (!accountInfo?.data) {
      throw new Error("Invalid message info");
    }
    return this.deserialize(accountInfo.data);
  }

  static async retrieveFromThread(
    connection: Connection,
    sender: PublicKey,
    receiver: PublicKey
  ) {
    const thread = await Thread.retrieve(connection, sender, receiver);
    let messageAccounts: PublicKey[] = [];
    for (let i = 0; i < thread.msgCount; i++) {
      const [acc] = await PublicKey.findProgramAddress(
        this.generateSeeds(i, sender, receiver),
        JABBER_ID
      );
      messageAccounts.push(acc);
    }
    const accountInfos = await connection.getMultipleAccountsInfo(
      messageAccounts
    );
    return accountInfos.map((info, i) =>
      info?.data
        ? { message: this.deserialize(info?.data), address: messageAccounts[i] }
        : undefined
    );
  }
}

export class GroupThread {
  tag: Tag;
  groupName: string;
  msgCount: number;
  destinationWallet: PublicKey;
  lamportsPerMessage: BN;
  bump: number;
  admins: PublicKey[];
  owner: PublicKey;
  mediaEnabled: boolean;

  static schema: Schema = new Map([
    [
      GroupThread,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["groupName", "string"],
          ["msgCount", "u32"],
          ["destinationWallet", [32]],
          ["lamportsPerMessage", "u64"],
          ["bump", "u8"],
          ["admins", [[32]]],
          ["owner", [32]],
          ["mediaEnabled", "u8"],
        ],
      },
    ],
  ]);

  constructor(obj: {
    groupName: string;
    msgCount: number;
    destinationWallet: Uint8Array;
    lamportsPerMessage: BN;
    bump: number;
    admins: Uint8Array[];
    owner: Uint8Array;
    mediaEnabled: number;
  }) {
    this.tag = Tag.GroupThread;
    this.groupName = obj.groupName;
    this.msgCount = obj.msgCount;
    this.destinationWallet = new PublicKey(obj.destinationWallet);
    this.lamportsPerMessage = obj.lamportsPerMessage;
    this.bump = obj.bump;
    this.admins = obj.admins.map((e) => new PublicKey(e));
    this.owner = new PublicKey(obj.owner);
    this.mediaEnabled = !!obj.mediaEnabled;
  }

  static deserialize(data: Buffer) {
    return deserialize(this.schema, GroupThread, data);
  }

  static generateSeeds(groupName: string, owner: PublicKey) {
    return [
      Buffer.from("group_thread"),
      Buffer.from(groupName),
      owner.toBuffer(),
    ];
  }

  static async getKey(groupName: string, owner: PublicKey) {
    const [groupThread] = await PublicKey.findProgramAddress(
      GroupThread.generateSeeds(groupName, owner),
      JABBER_ID
    );
    return groupThread;
  }

  static async retrieve(
    connection: Connection,
    groupName: string,
    owner: PublicKey
  ) {
    const groupThreadKey = await GroupThread.getKey(groupName, owner);

    const accountInfo = await connection.getAccountInfo(groupThreadKey);

    if (!accountInfo?.data) {
      throw new Error("Group thread not found");
    }

    return this.deserialize(accountInfo.data);
  }

  static async retrieveFromKey(connection: Connection, key: PublicKey) {
    const accountInfo = await connection.getAccountInfo(key);

    if (!accountInfo?.data) {
      throw new Error("Group thread not found");
    }

    return this.deserialize(accountInfo.data);
  }
}
