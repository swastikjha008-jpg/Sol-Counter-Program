import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import * as borsh from "borsh";

/** Mirrors `CounterAccount { count: u32 }` in src/lib.rs (4 bytes). */
export const COUNTER_SIZE = 4;

export const counterSchema = { struct: { count: "u32" } } as const;

export type CounterAccount = { count: number };

/**
 * Mirrors `InstructionType` in src/lib.rs.
 * Borsh layout: 1 byte variant tag + 4 byte little-endian u32.
 *   Increment(n) -> [0, n, 0, 0, 0]
 *   Decrement(n) -> [1, n, 0, 0, 0]
 */
export function encodeInstruction(kind: "increment" | "decrement", value: number): Buffer {
  const data = Buffer.alloc(5);
  data.writeUInt8(kind === "increment" ? 0 : 1, 0);
  data.writeUInt32LE(value, 1);
  return data;
}

/** Create a 4-byte counter account owned by the counter program. */
export async function createCounterAccount(
  connection: Connection,
  payer: Keypair,
  counter: Keypair,
  programId: PublicKey
): Promise<string> {
  const lamports = await connection.getMinimumBalanceForRentExemption(COUNTER_SIZE);
  const tx = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: counter.publicKey,
      lamports,
      space: COUNTER_SIZE,
      programId,
    })
  );
  return sendAndConfirmTransaction(connection, tx, [payer, counter]);
}

/** Increment or decrement the counter stored at `counterPubkey`. */
export async function updateCounter(
  connection: Connection,
  payer: Keypair,
  counterPubkey: PublicKey,
  programId: PublicKey,
  kind: "increment" | "decrement",
  value: number
): Promise<string> {
  const tx = new Transaction().add(
    new TransactionInstruction({
      keys: [{ pubkey: counterPubkey, isSigner: false, isWritable: true }],
      programId,
      data: encodeInstruction(kind, value),
    })
  );
  return sendAndConfirmTransaction(connection, tx, [payer]);
}

export async function readCounter(
  connection: Connection,
  counterPubkey: PublicKey
): Promise<number> {
  const info = await connection.getAccountInfo(counterPubkey);
  if (!info) throw new Error("Counter account not found");
  const counter = borsh.deserialize(counterSchema, info.data) as CounterAccount;
  return counter.count;
}
