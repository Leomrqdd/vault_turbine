# Anchor Vault

A Solana program (Anchor) that lets users create a personal vault, deposit and withdraw SOL, and close the vault to recover funds and rent.

## Features

- **Initialize** — Create a vault PDA for your wallet (one vault per user).
- **Deposit** — Send SOL into your vault.
- **Withdraw** — Withdraw SOL from your vault back to your wallet.
- **Close** — Close the vault and receive all remaining SOL plus rent exemption.

## Prerequisites

- [Rust](https://rustup.rs/) (with the version in `rust-toolchain.toml`)
- [Solana CLI](https://docs.solana.com/cli/install)
- [Anchor](https://www.anchor-lang.com/docs/installation)
- [Yarn](https://yarnpkg.com/)

## Setup

```bash
yarn install
anchor build
```

## Tests

Run the test suite (requires a solana local validator or surfpool):

```bash
anchor test
anchor test --skip-local-validator
```

## Project layout

- `programs/anchor-vault/src/lib.rs` — Program logic (initialize, deposit, withdraw, close).
- `tests/anchor-vault.ts` — TypeScript integration tests.

