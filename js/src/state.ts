import { PublicKey, Connection } from "@solana/web3.js";
import BN from "bn.js";
import { Schema, deserializeUnchecked, deserialize } from "borsh";
import { JABBER_ID } from "./bindings";
import { orderKeys } from "./utils";

export enum Tag {
  Uninitialized = 0,
  Profile = 1,
  Thread = 2,
  Message = 3,
  Jabber = 4,
  GroupThread = 5,
  GroupThreadIndex = 6,
  Subscription = 7,
}

export enum MessageType {
  Encrypted = 0,
  Unencrypted = 1,
  EncryptedImage = 2,
  UnencryptedImage = 3,
}

export class Profile {
  tag: Tag;
  bump: number;
  pictureHash: String;
  displayDomainName: String;
  bio: String;
  lamportsPerMessage: BN;
  allowDm: boolean;
  tipsSent: number;
  tipsReceived: number;

  static schema: Schema = new Map([
    [
      Profile,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["bump", "u8"],
          ["pictureHash", "string"],
          ["displayDomainName", "string"],
          ["bio", "string"],
          ["lamportsPerMessage", "u64"],
          ["allowDm", "u8"],
          ["tipsSent", "u64"],
          ["tipsReceived", "u64"],
        ],
      },
    ],
  ]);

  constructor(obj: {
    tag: Tag;
    bump: number;
    pictureHash: String;
    displayDomainName: String;
    bio: String;
    lamportsPerMessage: BN;
    allowDm: boolean;
    tipsSent: number;
    tipsReceived: number;
  }) {
    this.tag = Tag.Profile;
    this.bump = obj.bump;
    this.pictureHash = obj.pictureHash;
    this.displayDomainName = obj.displayDomainName;
    this.bio = obj.bio;
    this.lamportsPerMessage = obj.lamportsPerMessage;
    this.allowDm = obj.allowDm;
    this.tipsSent = obj.tipsSent;
    this.tipsReceived = obj.tipsReceived;
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
  lastMessageTime: BN;
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
          ["lastMessageTime", "u64"],
          ["bump", "u8"],
        ],
      },
    ],
  ]);

  constructor(obj: {
    msgCount: number;
    user1: Uint8Array;
    user2: Uint8Array;
    lastMessageTime: BN;
    bump: number;
  }) {
    this.tag = Tag.Thread;
    this.msgCount = obj.msgCount;
    this.user1 = new PublicKey(obj.user1);
    this.user2 = new PublicKey(obj.user2);
    this.lastMessageTime = obj.lastMessageTime;
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
  sender: PublicKey;
  repliesTo: PublicKey;
  likesCount: number;
  dislikesCount: number;
  msg: Uint8Array;

  static schema: Schema = new Map([
    [
      Message,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["kind", "u8"],
          ["timestamp", "u64"],
          ["sender", [32]],
          ["repliesTo", [32]],
          ["likesCount", "u16"],
          ["dislikesCount", "u16"],
          ["msg", ["u8"]],
        ],
      },
    ],
  ]);

  constructor(obj: {
    tag: Tag;
    kind: MessageType;
    timestamp: BN;
    sender: Uint8Array;
    repliesTo: Uint8Array;
    likesCount: number;
    dislikesCount: number;
    msg: Uint8Array;
  }) {
    this.tag = Tag.Message;
    this.kind = obj.kind;
    this.timestamp = obj.timestamp;
    this.sender = new PublicKey(obj.sender);
    this.repliesTo = new PublicKey(obj.repliesTo);
    this.likesCount = obj.likesCount;
    this.dislikesCount = obj.dislikesCount;
    this.msg = obj.msg;
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
  bump: number;
  visible: boolean;
  owner: PublicKey;
  lastMessageTime: BN;
  destinationWallet: PublicKey;
  msgCount: number;
  lamportsPerMessage: BN;
  mediaEnabled: boolean;
  adminOnly: boolean;
  groupPicHash: String;
  groupName: string;
  admins: PublicKey[];

  static schema: Schema = new Map([
    [
      GroupThread,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["bump", "u8"],
          ["visible", "u8"],
          ["owner", [32]],
          ["lastMessageTime", "u64"],
          ["destinationWallet", [32]],
          ["msgCount", "u32"],
          ["lamportsPerMessage", "u64"],
          ["mediaEnabled", "u8"],
          ["adminOnly", "u8"],
          ["groupPicHash", "string"],
          ["groupName", "string"],
          ["admins", [[32]]],
        ],
      },
    ],
  ]);

  constructor(obj: {
    tag: Tag;
    bump: number;
    visible: boolean;
    owner: PublicKey;
    lastMessageTime: BN;
    destinationWallet: PublicKey;
    msgCount: number;
    lamportsPerMessage: BN;
    mediaEnabled: boolean;
    adminOnly: boolean;
    groupPicHash: String;
    groupName: string;
    admins: PublicKey[];
  }) {
    this.tag = Tag.GroupThread;
    this.bump = obj.bump;
    this.visible = obj.visible;
    this.owner = new PublicKey(obj.owner);
    this.lastMessageTime = obj.lastMessageTime;
    this.destinationWallet = new PublicKey(obj.destinationWallet);
    this.msgCount = obj.msgCount;
    this.lamportsPerMessage = obj.lamportsPerMessage;
    this.mediaEnabled = !!obj.mediaEnabled;
    this.adminOnly = !!obj.adminOnly;
    this.groupPicHash = obj.groupPicHash;
    this.groupName = obj.groupName;
    this.admins = obj.admins.map((e) => new PublicKey(e));
  }

  static deserialize(data: Buffer) {
    return deserializeUnchecked(this.schema, GroupThread, data);
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

export class GroupThreadIndex {
  tag: number;
  groupThreadKey: Uint8Array;
  owner: Uint8Array;
  groupName: string;

  static schema: Schema = new Map([
    [
      GroupThreadIndex,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["groupThreadKey", [32]],
          ["owner", [32]],
          ["groupName", "string"],
        ],
      },
    ],
  ]);

  constructor(obj: {
    groupName: string;
    groupThreadKey: Uint8Array;
    owner: Uint8Array;
  }) {
    this.tag = Tag.GroupThreadIndex;
    this.groupName = obj.groupName;
    this.groupThreadKey = obj.groupThreadKey;
    this.owner = obj.owner;
  }

  static deserialize(data: Buffer) {
    return deserializeUnchecked(this.schema, GroupThreadIndex, data);
  }

  static generateSeeds(
    groupName: string,
    owner: PublicKey,
    groupThreadKey: PublicKey
  ) {
    return [
      Buffer.from("group_thread_index"),
      Buffer.from(groupName),
      owner.toBuffer(),
      groupThreadKey.toBuffer(),
    ];
  }

  static async getKey(
    groupName: string,
    owner: PublicKey,
    groupThreadKey: PublicKey
  ) {
    const [groupThreadIndex] = await PublicKey.findProgramAddress(
      GroupThreadIndex.generateSeeds(groupName, owner, groupThreadKey),
      JABBER_ID
    );
    return groupThreadIndex;
  }

  static async retrieve(
    connection: Connection,
    groupName: string,
    owner: PublicKey,
    groupThreadKey: PublicKey
  ) {
    const groupThreadIndexKey = await GroupThreadIndex.getKey(
      groupName,
      owner,
      groupThreadKey
    );

    const accountInfo = await connection.getAccountInfo(groupThreadIndexKey);

    if (!accountInfo?.data) {
      throw new Error("Group index not found");
    }

    return this.deserialize(accountInfo.data);
  }

  static async retrieveFromKey(connection: Connection, key: PublicKey) {
    const accountInfo = await connection.getAccountInfo(key);

    if (!accountInfo?.data) {
      throw new Error("Group index not found");
    }

    return this.deserialize(accountInfo.data);
  }
}

export class Subscription {
  tag: number;
  subscriber: Uint8Array;
  subscribedTo: Uint8Array;

  static schema: Schema = new Map([
    [
      Subscription,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["subscriber", [32]],
          ["subscribedTo", [32]],
        ],
      },
    ],
  ]);

  constructor(obj: { subscriber: Uint8Array; subscribedTo: Uint8Array }) {
    this.tag = Tag.Subscription;
    this.subscriber = obj.subscriber;
    this.subscribedTo = obj.subscribedTo;
  }

  static deserialize(data: Buffer) {
    return deserializeUnchecked(this.schema, Subscription, data);
  }

  static generateSeeds(subscriber: PublicKey, subscribedTo: PublicKey) {
    return [
      Buffer.from("subscription"),
      subscriber.toBuffer(),
      subscribedTo.toBuffer(),
    ];
  }

  static async getKey(subscriber: PublicKey, subscribedTo: PublicKey) {
    const [subscriptionKey] = await PublicKey.findProgramAddress(
      Subscription.generateSeeds(subscriber, subscribedTo),
      JABBER_ID
    );
    return subscriptionKey;
  }
}
