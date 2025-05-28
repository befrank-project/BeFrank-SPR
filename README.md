
# BeFrank Token (BFR)

BeFrank is a socially-oriented Solana token where each transaction automatically contributes to a Welfare Fund, while the recipient receives the full amount. This repository contains the smart contract and test suite built using the Anchor framework.

## 📦 Features

- SPL token with 6 decimals
- Sender-paid transaction fees:
  - 0.5% to Welfare Fund
  - 0.25% to Staking Pool
  - 0.25% to Ecosystem Fund
- Anchor-based Solana smart contract
- Typescript Mocha test stub included

## 📜 Deployment Instructions

### Prerequisites

- Node.js (v16 or later)
- Yarn
- Solana CLI (`solana --version`)
- Anchor CLI (`anchor --version`)

### 1. Install dependencies

```bash
yarn install
```

### 2. Build the program

```bash
anchor build
```

### 3. Deploy to Devnet

```bash
solana config set --url devnet
anchor deploy
```

### 4. Run Tests

```bash
anchor test
```

## 🔧 Accounts

- `initialize`: Initializes the token and mints total supply
- `transfer_with_fee`: Sends tokens and splits the 1% fee to predefined accounts

## 📁 Structure

- `programs/befrank_token/src/lib.rs` – main smart contract logic
- `tests/befrank_test.ts` – integration test

## 📄 License

MIT License

## 🚀 Deployment Instructions

### Prerequisites
- Solana CLI
- Anchor CLI
- Node.js + Yarn

### Devnet Deployment
```bash
solana config set --url https://api.devnet.solana.com
solana-keygen new --outfile ~/.config/solana/befrank-devnet.json
solana config set --keypair ~/.config/solana/befrank-devnet.json
solana airdrop 2
anchor build
anchor deploy
```

### Mainnet Deployment
```bash
solana config set --url https://api.mainnet-beta.solana.com
solana config set --keypair ~/.config/solana/befrank-mainnet.json
# Ensure wallet is funded manually
anchor build
anchor deploy
```

### Run Tests
```bash
anchor test
```
