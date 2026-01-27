# Solana Token Project

A robust SPL Token project built on the Solana Blockchain using the Anchor framework. This project demonstrates how to create, mint, and transfer custom tokens with strict security and structured event logging.

## 🚀 Overview

This project implements a standard SPL Token contract that allows you to:
- **Initialize a new Token Mint** with custom decimals.
- **Mint new tokens** to specific user accounts.
- **Transfer tokens** securely between accounts.

It is designed as a foundational template for any Solana project requiring tokenomics.

## ✨ Features

- **Initialize Mint**: Create a brand new SPL token with full control over decimals and authorities.
- **Minting Authority**: securely mint new tokens to any destination wallet.
- **Secure Transfers**: Standard `transfer` instruction to move tokens between accounts.
- **Event Logging**: Custom Anchor events (`MintInitialized`, `TokensMinted`, `TokensTransferred`) for easy indexing and frontend tracking.

## 🛠️ Prerequisites

Ensure you have the following tools installed on your system:

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Solana CLI**: [Install Solana Tool Suite](https://docs.solana.com/cli/install-solana-cli-tools)
- **Anchor**: [Install Anchor Framework](https://www.anchor-lang.com/docs/installation)
- **Node.js & Yarn**: Required for running tests and scripts.

## 📦 Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/rajinder1310/solana-spl-token-project.git
    cd solana-spl-token-project
    ```

2.  **Install dependencies:**
    ```bash
    yarn install
    ```

## 💻 Usage

### Build the Program
Compile the Rust smart contract:
```bash
anchor build
```

### Run Tests
Execute the comprehensive test suite to verify functionality:
```bash
anchor test
```

### Deploy to Devnet
Use the included helper script to deploy your program to the Solana Devnet.
```bash
./scripts/deploy.sh
```
*Note: Make sure you have a valid keypair configured in your environment.*

### Verify Program
After deployment, you can verify your program on the Solana Explorer:
```bash
./scripts/verify.sh
```

## 📂 Project Structure

- **`programs/`**: Contains the Rust smart contract logic (`lib.rs`).
- **`tests/`**: TypeScript tests to verify contract functionality.
- **`scripts/`**: Helper shell scripts for deployment and verification.
- **`Anchor.toml`**: Main configuration file for the project settings and cluster details.

## 📄 License
This project is open-source and available under the MIT License.
