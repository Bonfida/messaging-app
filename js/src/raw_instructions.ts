// This file is auto-generated. DO NOT EDIT
import BN from "bn.js";
import { Schema, serialize } from "borsh";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";

export interface AccountKey {
  pubkey: PublicKey;
  isSigner: boolean;
  isWritable: boolean;
}
export class deleteMessageInstruction {
  tag: number;
  messageIndex: number;
  static schema: Schema = new Map([
    [
      deleteMessageInstruction,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["messageIndex", "u32"],
        ],
      },
    ],
  ]);
  constructor(obj: { messageIndex: number }) {
    this.tag = 10;
    this.messageIndex = obj.messageIndex;
  }
  serialize(): Uint8Array {
    return serialize(deleteMessageInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    sender: PublicKey,
    receiver: PublicKey,
    message: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: sender,
      isSigner: true,
      isWritable: true,
    });
    keys.push({
      pubkey: receiver,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: message,
      isSigner: false,
      isWritable: true,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
export class sendMessageGroupInstruction {
  tag: number;
  kind: number;
  repliesTo: Uint8Array;
  adminIndex: number;
  groupName: string;
  message: number[];
  static schema: Schema = new Map([
    [
      sendMessageGroupInstruction,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["kind", "u8"],
          ["repliesTo", [32]],
          ["adminIndex", "u8"],
          ["groupName", "string"],
          ["message", ["u8"]],
        ],
      },
    ],
  ]);
  constructor(obj: {
    kind: number;
    repliesTo: Uint8Array;
    adminIndex: number;
    groupName: string;
    message: number[];
  }) {
    this.tag = 6;
    this.kind = obj.kind;
    this.repliesTo = obj.repliesTo;
    this.adminIndex = obj.adminIndex;
    this.groupName = obj.groupName;
    this.message = obj.message;
  }
  serialize(): Uint8Array {
    return serialize(sendMessageGroupInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    systemProgram: PublicKey,
    sender: PublicKey,
    groupThread: PublicKey,
    destinationWallet: PublicKey,
    message: PublicKey,
    solVault: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: systemProgram,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: sender,
      isSigner: true,
      isWritable: true,
    });
    keys.push({
      pubkey: groupThread,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: destinationWallet,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: message,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: solVault,
      isSigner: false,
      isWritable: true,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
export class sendMessageInstruction {
  tag: number;
  kind: number;
  repliesTo: Uint8Array;
  message: number[];
  static schema: Schema = new Map([
    [
      sendMessageInstruction,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["kind", "u8"],
          ["repliesTo", [32]],
          ["message", ["u8"]],
        ],
      },
    ],
  ]);
  constructor(obj: { kind: number; repliesTo: Uint8Array; message: number[] }) {
    this.tag = 3;
    this.kind = obj.kind;
    this.repliesTo = obj.repliesTo;
    this.message = obj.message;
  }
  serialize(): Uint8Array {
    return serialize(sendMessageInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    systemProgram: PublicKey,
    sender: PublicKey,
    receiver: PublicKey,
    thread: PublicKey,
    receiverProfile: PublicKey,
    message: PublicKey,
    solVault: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: systemProgram,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: sender,
      isSigner: true,
      isWritable: true,
    });
    keys.push({
      pubkey: receiver,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: thread,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: receiverProfile,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: message,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: solVault,
      isSigner: false,
      isWritable: true,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
export class createProfileInstruction {
  tag: number;
  pictureHash: string;
  displayDomainName: string;
  bio: string;
  lamportsPerMessage: BN;
  static schema: Schema = new Map([
    [
      createProfileInstruction,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["pictureHash", "string"],
          ["displayDomainName", "string"],
          ["bio", "string"],
          ["lamportsPerMessage", "u64"],
        ],
      },
    ],
  ]);
  constructor(obj: {
    pictureHash: string;
    displayDomainName: string;
    bio: string;
    lamportsPerMessage: BN;
  }) {
    this.tag = 0;
    this.pictureHash = obj.pictureHash;
    this.displayDomainName = obj.displayDomainName;
    this.bio = obj.bio;
    this.lamportsPerMessage = obj.lamportsPerMessage;
  }
  serialize(): Uint8Array {
    return serialize(createProfileInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    systemProgram: PublicKey,
    profile: PublicKey,
    profileOwner: PublicKey,
    feePayer: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: systemProgram,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: profile,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: profileOwner,
      isSigner: true,
      isWritable: true,
    });
    keys.push({
      pubkey: feePayer,
      isSigner: true,
      isWritable: true,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
export class deleteGroupMessageInstruction {
  tag: number;
  messageIndex: number;
  owner: Uint8Array;
  adminIndex: number;
  groupName: string;
  static schema: Schema = new Map([
    [
      deleteGroupMessageInstruction,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["messageIndex", "u32"],
          ["owner", [32]],
          ["adminIndex", "u8"],
          ["groupName", "string"],
        ],
      },
    ],
  ]);
  constructor(obj: {
    messageIndex: number;
    owner: Uint8Array;
    adminIndex: number;
    groupName: string;
  }) {
    this.tag = 11;
    this.messageIndex = obj.messageIndex;
    this.owner = obj.owner;
    this.adminIndex = obj.adminIndex;
    this.groupName = obj.groupName;
  }
  serialize(): Uint8Array {
    return serialize(deleteGroupMessageInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    groupThread: PublicKey,
    message: PublicKey,
    feePayer: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: groupThread,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: message,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: feePayer,
      isSigner: true,
      isWritable: true,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
export class sendTipInstruction {
  tag: number;
  amount: BN;
  static schema: Schema = new Map([
    [
      sendTipInstruction,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["amount", "u64"],
        ],
      },
    ],
  ]);
  constructor(obj: { amount: BN }) {
    this.tag = 12;
    this.amount = obj.amount;
  }
  serialize(): Uint8Array {
    return serialize(sendTipInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    splTokenProgram: PublicKey,
    senderProfile: PublicKey,
    sender: PublicKey,
    receiverProfile: PublicKey,
    receiver: PublicKey,
    tokenSource: PublicKey,
    tokenDestination: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: splTokenProgram,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: senderProfile,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: sender,
      isSigner: true,
      isWritable: true,
    });
    keys.push({
      pubkey: receiverProfile,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: receiver,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: tokenSource,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: tokenDestination,
      isSigner: false,
      isWritable: true,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
export class createThreadInstruction {
  tag: number;
  senderKey: Uint8Array;
  receiverKey: Uint8Array;
  static schema: Schema = new Map([
    [
      createThreadInstruction,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["senderKey", [32]],
          ["receiverKey", [32]],
        ],
      },
    ],
  ]);
  constructor(obj: { senderKey: Uint8Array; receiverKey: Uint8Array }) {
    this.tag = 1;
    this.senderKey = obj.senderKey;
    this.receiverKey = obj.receiverKey;
  }
  serialize(): Uint8Array {
    return serialize(createThreadInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    systemProgram: PublicKey,
    thread: PublicKey,
    feePayer: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: systemProgram,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: thread,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: feePayer,
      isSigner: true,
      isWritable: true,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
export class addAdminToGroupInstruction {
  tag: number;
  adminAddress: Uint8Array;
  static schema: Schema = new Map([
    [
      addAdminToGroupInstruction,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["adminAddress", [32]],
        ],
      },
    ],
  ]);
  constructor(obj: { adminAddress: Uint8Array }) {
    this.tag = 7;
    this.adminAddress = obj.adminAddress;
  }
  serialize(): Uint8Array {
    return serialize(addAdminToGroupInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    groupThread: PublicKey,
    groupOwner: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: groupThread,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: groupOwner,
      isSigner: true,
      isWritable: true,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
export class editGroupThreadInstruction {
  tag: number;
  visible: number;
  destinationWallet: Uint8Array;
  lamportsPerMessage: BN;
  owner: Uint8Array;
  mediaEnabled: number;
  adminOnly: number;
  groupPicHash: string;
  static schema: Schema = new Map([
    [
      editGroupThreadInstruction,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["visible", "u8"],
          ["destinationWallet", [32]],
          ["lamportsPerMessage", "u64"],
          ["owner", [32]],
          ["mediaEnabled", "u8"],
          ["adminOnly", "u8"],
          ["groupPicHash", "string"],
        ],
      },
    ],
  ]);
  constructor(obj: {
    visible: number;
    destinationWallet: Uint8Array;
    lamportsPerMessage: BN;
    owner: Uint8Array;
    mediaEnabled: number;
    adminOnly: number;
    groupPicHash: string;
  }) {
    this.tag = 5;
    this.visible = obj.visible;
    this.destinationWallet = obj.destinationWallet;
    this.lamportsPerMessage = obj.lamportsPerMessage;
    this.owner = obj.owner;
    this.mediaEnabled = obj.mediaEnabled;
    this.adminOnly = obj.adminOnly;
    this.groupPicHash = obj.groupPicHash;
  }
  serialize(): Uint8Array {
    return serialize(editGroupThreadInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    groupOwner: PublicKey,
    groupThread: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: groupOwner,
      isSigner: true,
      isWritable: true,
    });
    keys.push({
      pubkey: groupThread,
      isSigner: false,
      isWritable: true,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
export class createGroupIndexInstruction {
  tag: number;
  groupName: string;
  groupThreadKey: Uint8Array;
  owner: Uint8Array;
  static schema: Schema = new Map([
    [
      createGroupIndexInstruction,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["groupName", "string"],
          ["groupThreadKey", [32]],
          ["owner", [32]],
        ],
      },
    ],
  ]);
  constructor(obj: {
    groupName: string;
    groupThreadKey: Uint8Array;
    owner: Uint8Array;
  }) {
    this.tag = 9;
    this.groupName = obj.groupName;
    this.groupThreadKey = obj.groupThreadKey;
    this.owner = obj.owner;
  }
  serialize(): Uint8Array {
    return serialize(createGroupIndexInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    systemProgram: PublicKey,
    groupThreadIndex: PublicKey,
    feePayer: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: systemProgram,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: groupThreadIndex,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: feePayer,
      isSigner: true,
      isWritable: true,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
export class setUserProfileInstruction {
  tag: number;
  pictureHash: string;
  displayDomainName: string;
  bio: string;
  lamportsPerMessage: BN;
  allowDm: number;
  static schema: Schema = new Map([
    [
      setUserProfileInstruction,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["pictureHash", "string"],
          ["displayDomainName", "string"],
          ["bio", "string"],
          ["lamportsPerMessage", "u64"],
          ["allowDm", "u8"],
        ],
      },
    ],
  ]);
  constructor(obj: {
    pictureHash: string;
    displayDomainName: string;
    bio: string;
    lamportsPerMessage: BN;
    allowDm: number;
  }) {
    this.tag = 2;
    this.pictureHash = obj.pictureHash;
    this.displayDomainName = obj.displayDomainName;
    this.bio = obj.bio;
    this.lamportsPerMessage = obj.lamportsPerMessage;
    this.allowDm = obj.allowDm;
  }
  serialize(): Uint8Array {
    return serialize(setUserProfileInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    profileOwner: PublicKey,
    profile: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: profileOwner,
      isSigner: true,
      isWritable: true,
    });
    keys.push({
      pubkey: profile,
      isSigner: false,
      isWritable: true,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
export class removeAdminFromGroupInstruction {
  tag: number;
  adminAddress: Uint8Array;
  adminIndex: BN;
  static schema: Schema = new Map([
    [
      removeAdminFromGroupInstruction,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["adminAddress", [32]],
          ["adminIndex", "u64"],
        ],
      },
    ],
  ]);
  constructor(obj: { adminAddress: Uint8Array; adminIndex: BN }) {
    this.tag = 8;
    this.adminAddress = obj.adminAddress;
    this.adminIndex = obj.adminIndex;
  }
  serialize(): Uint8Array {
    return serialize(removeAdminFromGroupInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    groupThread: PublicKey,
    groupOwner: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: groupThread,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: groupOwner,
      isSigner: true,
      isWritable: true,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
export class createGroupThreadInstruction {
  tag: number;
  visible: number;
  groupName: string;
  destinationWallet: Uint8Array;
  lamportsPerMessage: BN;
  admins: Uint8Array[];
  owner: Uint8Array;
  mediaEnabled: number;
  adminOnly: number;
  static schema: Schema = new Map([
    [
      createGroupThreadInstruction,
      {
        kind: "struct",
        fields: [
          ["tag", "u8"],
          ["visible", "u8"],
          ["groupName", "string"],
          ["destinationWallet", [32]],
          ["lamportsPerMessage", "u64"],
          ["admins", [[32]]],
          ["owner", [32]],
          ["mediaEnabled", "u8"],
          ["adminOnly", "u8"],
        ],
      },
    ],
  ]);
  constructor(obj: {
    visible: number;
    groupName: string;
    destinationWallet: Uint8Array;
    lamportsPerMessage: BN;
    admins: Uint8Array[];
    owner: Uint8Array;
    mediaEnabled: number;
    adminOnly: number;
  }) {
    this.tag = 4;
    this.visible = obj.visible;
    this.groupName = obj.groupName;
    this.destinationWallet = obj.destinationWallet;
    this.lamportsPerMessage = obj.lamportsPerMessage;
    this.admins = obj.admins;
    this.owner = obj.owner;
    this.mediaEnabled = obj.mediaEnabled;
    this.adminOnly = obj.adminOnly;
  }
  serialize(): Uint8Array {
    return serialize(createGroupThreadInstruction.schema, this);
  }
  getInstruction(
    programId: PublicKey,
    systemProgram: PublicKey,
    groupThread: PublicKey,
    feePayer: PublicKey
  ): TransactionInstruction {
    const data = Buffer.from(this.serialize());
    let keys: AccountKey[] = [];
    keys.push({
      pubkey: systemProgram,
      isSigner: false,
      isWritable: false,
    });
    keys.push({
      pubkey: groupThread,
      isSigner: false,
      isWritable: true,
    });
    keys.push({
      pubkey: feePayer,
      isSigner: true,
      isWritable: true,
    });
    return new TransactionInstruction({
      keys,
      programId,
      data,
    });
  }
}
