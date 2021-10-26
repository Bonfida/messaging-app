import {
  Connection,
  TransactionInstruction,
  Keypair,
  Transaction,
  PublicKey,
} from "@solana/web3.js";

import ed2curve from "./ed2curve";
import nacl from "tweetnacl";

export const signAndSendTransactionInstructions = async (
  // sign and send transaction
  connection: Connection,
  signers: Array<Keypair>,
  feePayer: Keypair,
  txInstructions: Array<TransactionInstruction>
): Promise<string> => {
  const tx = new Transaction();
  tx.feePayer = feePayer.publicKey;
  signers.push(feePayer);
  tx.add(...txInstructions);
  return await connection.sendTransaction(tx, signers, {
    skipPreflight: false,
  });
};

export const orderKeys = (key1: PublicKey, key2: PublicKey) => {
  if (key1.toBase58() < key2.toBase58()) {
    return [key1, key2];
  }
  return [key2, key1];
};

export const encryptMessage = (
  msg: Uint8Array,
  dhKeys: { publicKey: Buffer; secretKey: Buffer },
  rPublicKey: PublicKey,
  nonce: Uint8Array
): Uint8Array => {
  const dhrPk = ed2curve.convertPublicKey(rPublicKey);
  return nacl.box(msg, nonce, dhrPk, dhKeys.secretKey);
};

export const decryptMessage = (
  msg: Uint8Array,
  dhKeys: { publicKey: Buffer; secretKey: Buffer },
  fromPk: PublicKey,
  nonce: Uint8Array
): Uint8Array | null => {
  const dhrPk = ed2curve.convertPublicKey(fromPk);
  return nacl.box.open(msg, nonce, dhrPk, dhKeys.secretKey);
};
