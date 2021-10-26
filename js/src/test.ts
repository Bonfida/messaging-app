import { Connection } from "@solana/web3.js";
import { JABBER_ID } from "./instructions";
import BN from "bn.js";
import base58 from "bs58";

const connection = new Connection(
  "https://solana--mainnet.datahub.figment.io/apikey/f5cf21bd026fd1327dcec7ee9cc917a6"
);

export const test = async () => {
  const filters = [
    {
      memcmp: {
        offset: 0,
        bytes: base58.encode(new BN(3).toBuffer()),
      },
    },
  ];
  const t = await connection.getProgramAccounts(JABBER_ID, { filters });
  console.log(t.length);
};
test();
