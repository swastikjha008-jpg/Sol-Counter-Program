import { beforeAll, expect, test } from "bun:test";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { createCounterAccount, readCounter, updateCounter } from "./counter";

// Run against a local validator with the program deployed:
//   solana-test-validator
//   cargo build-sbf && solana program deploy target/deploy/sol_counter_program.so
//   PROGRAM_ID=<printed id> bun test
const RPC_URL = process.env.RPC_URL ?? "http://127.0.0.1:8899";
const PROGRAM_ID = process.env.PROGRAM_ID;
if (!PROGRAM_ID) throw new Error("Set PROGRAM_ID to the deployed counter program id");

const programId = new PublicKey(PROGRAM_ID);
const connection = new Connection(RPC_URL, "confirmed");
const adminKeypair = Keypair.generate();
const counterAccountKeypair = Keypair.generate();

beforeAll(async () => {
  const sig = await connection.requestAirdrop(adminKeypair.publicKey, 2 * LAMPORTS_PER_SOL);
  await connection.confirmTransaction(sig, "confirmed");
  await createCounterAccount(connection, adminKeypair, counterAccountKeypair, programId);
});

test("counter starts at zero", async () => {
  expect(await readCounter(connection, counterAccountKeypair.publicKey)).toBe(0);
});

test("counter does increase", async () => {
  const txHash = await updateCounter(
    connection, adminKeypair, counterAccountKeypair.publicKey, programId, "increment", 1
  );
  console.log(txHash);
  expect(await readCounter(connection, counterAccountKeypair.publicKey)).toBe(1);
});

test("counter does decrease", async () => {
  await updateCounter(
    connection, adminKeypair, counterAccountKeypair.publicKey, programId, "increment", 9
  );
  await updateCounter(
    connection, adminKeypair, counterAccountKeypair.publicKey, programId, "decrement", 4
  );
  expect(await readCounter(connection, counterAccountKeypair.publicKey)).toBe(6);
});
