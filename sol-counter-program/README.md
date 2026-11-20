# sol-counter-program

A minimal Solana program (Rust, `solana-program` 1.18) that lets a user **increment or decrement** a counter stored in any program-owned account.

## How it works

- **Account data** (4 bytes): `CounterAccount { count: u32 }` (Borsh).
- **Instruction data** (5 bytes): Borsh `InstructionType`
  - `Increment(n)` -> `[0, n(LE u32)]`
  - `Decrement(n)` -> `[1, n(LE u32)]`
- **Accounts**: `[0]` counter account (writable, owned by this program).
- Overflow/underflow returns `ArithmeticOverflow` instead of wrapping.

## Test

```bash
cargo test
```

## Client (TypeScript)

`client/` has helpers (`counter.ts`) and tests (`counter.test.ts`) using `@solana/web3.js` + `borsh`.

```bash
solana-test-validator                       # terminal 1
cargo build-sbf
solana program deploy target/deploy/sol_counter_program.so   # prints PROGRAM_ID
cd client && bun install
PROGRAM_ID=<id> bun test                    # RPC_URL defaults to localhost
```

## Build & deploy (Devnet)

```bash
cargo build-sbf
solana config set --url devnet
solana program deploy target/deploy/sol_counter_program.so
```

Create the counter account owned by the deployed program (4 bytes, rent-exempt) and send `Increment`/`Decrement` instructions to it.

## License

MIT
