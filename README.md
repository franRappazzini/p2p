# P2P Escrow - Solana Program

**Client Repository**: [https://github.com/franRappazzini/p2p-client](https://github.com/franRappazzini/p2p-client)

A peer-to-peer (P2P) escrow program built on Solana using the Anchor framework. This project facilitates secure SPL token exchanges between buyers and sellers, with dispute resolution mechanisms and protection for both parties.

## Auditware Radar audit

<img src="https://img.shields.io/github/actions/workflow/status/franRappazzini/p2p/radar.yaml">

## 📋 Table of Contents

- [How Solana Powers the Core of Our Project](#how-solana-powers-the-core-of-our-project)
- [Overview](#-overview)
- [Features](#-features)
- [Project Structure](#-project-structure)
- [Program Architecture](#-program-architecture)
- [Program Methods](#-program-methods)
- [States and Accounts](#-states-and-accounts)
- [Events](#-events)
- [Tests](#-tests)
- [Installation and Setup](#-installation-and-setup)
- [Local Development](#-local-development)
- [Deployment](#-deployment)
- [License](#-license)

# How Solana Powers the Core of Our Project

Solana is not just a blockchain choice for our P2P exchange platform—it's the fundamental enabler that makes decentralized fiat-to-crypto trading actually viable and competitive with centralized solutions.

## Speed Enables Real-Time Trading

P2P exchanges require immediate transaction finality. When a buyer confirms fiat payment, the crypto needs to be released from escrow instantly. Solana's 400ms block times and sub-second finality make this possible. Users experience the responsiveness they expect from modern financial applications, with transactions confirming almost immediately. This speed is crucial for building trust in the P2P exchange process, where any delay creates uncertainty and friction.

## Low Fees Make Micro-Transactions Feasible

Our platform targets global adoption, including users in emerging markets who may trade small amounts. With transaction costs averaging $0.00025, Solana makes every trade economically viable regardless of size. Whether someone is trading $10 or $10,000, the fees remain negligible. This truly democratizes access to cryptocurrency, ensuring that cost is never a barrier to entry for anyone, anywhere in the world.

## Program Architecture for Trustless Escrow

Our smart contract leverages Solana's account model and Program Derived Addresses (PDAs) to create secure, trustless escrow mechanics. When an order is created, funds are locked in a PDA controlled by our program logic—neither party can access them unilaterally. The escrow automatically releases crypto to the buyer only when conditions are met, or returns it to the seller if the trade is cancelled. This eliminates counterparty risk without requiring a trusted third party, making peer-to-peer trading truly safe and decentralized.

## Scalability for Global Adoption

Solana's capacity to handle 65,000+ transactions per second means our platform can scale globally without congestion or degraded performance. As trading volume grows, the network maintains consistent speed and costs. This scalability is critical for a P2P platform where thousands of users might be creating orders, updating escrows, and completing trades simultaneously across different regions and time zones.

**In summary:** Solana's combination of speed, cost-efficiency, and robust programming capabilities makes decentralized P2P fiat-crypto exchange not just possible, but practical and scalable. The network provides the infrastructure needed to deliver a smooth, affordable, and trustless experience that can serve users globally without compromise.

## 🎯 Overview

This Solana program implements a decentralized escrow system that enables secure P2P transactions between users. The typical flow includes:

1. **Seller** deposits tokens into an escrow account
2. **Buyer** makes the fiat payment off-chain
3. **Buyer** marks the escrow as paid
4. **Seller** releases the tokens to the buyer after confirming the fiat payment
5. If there are issues, either party can create a dispute

The program includes:

- Configurable fee system
- Configurable deadlines for fiat payments and disputes
- Dispute mechanism with security deposit
- Fraud protection with signature validation
- Multi SPL token management

## ✨ Features

- ✅ **Secure Escrow**: Tokens locked until both parties fulfill their obligations
- ✅ **Dispute System**: Two-level mechanism (dispute and re-dispute)
- ✅ **Direct Release**: Seller releases tokens after confirming fiat payment
- ✅ **Configurable Deadlines**: Time limits for payments and disputes
- ✅ **Flexible Fees**: Configurable basis points (BPS) system
- ✅ **Multi-Token**: Support for any SPL token
- ✅ **Events**: Event emission for tracking and monitoring
- ✅ **Fund Management**: Vault system to store fees

## 📁 Estructura del Proyecto

```
p2p/
├── programs/p2p/
│   └── src/
│       ├── lib.rs                    # Program entry point
│       ├── constants.rs              # System constants
│       ├── errors.rs                 # Custom errors
│       ├── events.rs                 # Emitted events
│       ├── instructions/             # Instruction logic
│       │   ├── mod.rs
│       │   ├── initialize.rs         # Global configuration initialization
│       │   ├── update_global_config.rs # Update global configuration
│       │   ├── create_escrow.rs      # Escrow creation
│       │   ├── mark_escrow_as_paid.rs # Mark fiat payment
│       │   ├── release_tokens_in_escrow.rs # Release tokens
│       │   ├── cancel_escrow.rs      # Cancel escrow
│       │   ├── create_dispute.rs     # Create dispute
│       │   ├── resolve_dispute.rs    # Resolve dispute
│       │   └── withdraw_spl.rs       # Withdraw fees
│       └── states/                   # Account definitions
│           ├── mod.rs
│           ├── global_config.rs      # Global configuration
│           ├── escrow.rs             # Escrow state
│           └── mint_vault.rs         # Token vault
├── tests/
│   ├── p2p.test.ts                   # Main tests
│   └── utils/                        # Testing utilities
│       ├── accounts.ts
│       ├── constants.ts
│       ├── events.ts
│       ├── functions.ts
│       └── parsers.ts
├── target/
│   ├── idl/p2p.json                  # Generated IDL
│   └── types/p2p.ts                  # TypeScript types
├── Anchor.toml                       # Anchor configuration
├── Cargo.toml                        # Rust dependencies
├── package.json                      # Node dependencies
└── tsconfig.json                     # TypeScript configuration
```

## 🏗️ Program Architecture

### Main Accounts

1. **GlobalConfig**: Program global configuration

   - Authority (administrator)
   - Escrow counter
   - Fee and deadline parameters
   - Available funds for withdrawal

2. **Escrow**: Represents a P2P transaction

   - Unique ID
   - Seller and buyer
   - Token mint and amount
   - Current state (Open, FiatPaid, Dispute, ReDispute)
   - Dispute information

3. **MintVault**: Stores tokens and fees per mint
   - Total deposited amount
   - Available amount for withdrawal

## 🔧 Program Methods

### 1. `initialize`

Initializes the program's global configuration (only once).

```rust
pub fn initialize(
    ctx: Context<Initialize>,
    fee_bps: u16,                    // Fee in basis points (e.g., 100 = 1%)
    fiat_deadline_secs: i64,         // Deadline for fiat payment in seconds
    dispute_deadline_secs: i64,      // Deadline to create dispute in seconds
    dispute_fee_escrow: u64,         // Required deposit for disputes (lamports)
) -> Result<()>
```

**Parameters:**

- `fee_bps`: System fee (e.g., 100 = 1%, 250 = 2.5%)
- `fiat_deadline_secs`: Time limit for buyer to pay (e.g., 1800 = 30 min)
- `dispute_deadline_secs`: Minimum time before being able to dispute (e.g., 43200 = 12 hours)
- `dispute_fee_escrow`: Deposit in lamports to create a dispute

**Usage:**
Only the authority can call this function once during the program's lifecycle.

---

### 2. `create_escrow`

Creates a new escrow by depositing tokens from the seller.

```rust
pub fn create_escrow(
    ctx: Context<CreateEscrow>,
    amount: u64,                     // Amount of tokens to deposit
) -> Result<()>
```

**Process:**

1. Transfers tokens from seller's account to vault
2. Creates an Escrow account with `Open` state
3. Increments global escrow counter
4. Emits `EscrowCreated` event

**Requirements:**

- Seller must have sufficient tokens
- Buyer must be specified in context accounts

---

### 3. `mark_escrow_as_paid`

The buyer marks the escrow as paid after transferring fiat off-chain.

```rust
pub fn mark_escrow_as_paid(
    ctx: Context<MarkEscrowAsPaid>,
    escrow_id: u64,
) -> Result<()>
```

**Process:**

1. Verifies the caller is the buyer
2. Changes escrow state to `FiatPaid`
3. Records the timestamp
4. Emits `MarkEscrowAsPaid` event

**Requirements:**

- Only the buyer can call this function
- Escrow must be in `Open` state

---

### 4. `release_tokens_in_escrow`

Releases tokens to the buyer. Called by the seller after confirming the fiat payment.

```rust
pub fn release_tokens_in_escrow(
    ctx: Context<ReleaseTokensInEscrow>,
    escrow_id: u64,
) -> Result<()>
```

**Process:**

1. Verifies the caller is the seller
2. Calculates and deducts the fee
3. Transfers tokens to buyer
4. Updates vault with fees
5. Closes escrow account
6. Emits `TokensReleased` event

**Requirements:**

- Only the seller can call this function
- Escrow must be in `FiatPaid` state

---

### 5. `cancel_escrow`

Cancels an escrow and returns tokens to the seller.

```rust
pub fn cancel_escrow(
    ctx: Context<CancelEscrow>,
    escrow_id: u64,
) -> Result<()>
```

**Process:**

1. Verifies that the fiat payment deadline has passed
2. Verifies the state is `Open`
3. Returns tokens to seller
4. Closes escrow account
5. Emits `EscrowCancelled` event

**Requirements:**

- Only the seller can cancel
- `fiat_deadline_secs` must have elapsed since creation
- State must be `Open` (not paid)

---

### 6. `create_dispute`

Creates a dispute on an escrow (can be dispute or re-dispute).

```rust
pub fn create_dispute(
    ctx: Context<CreateDispute>,
    escrow_id: u64,
) -> Result<()>
```

**Process:**

1. Verifies that the dispute deadline has passed
2. Charges the dispute deposit in lamports
3. Changes state to `Dispute` or `ReDispute`
4. Records who disputes (seller or buyer)
5. Emits `DisputeCreated` event

**Possible states:**

- `FiatPaid` → `Dispute`: First dispute
- `Dispute` → `ReDispute`: Counter-dispute from the other party

**Requirements:**

- `dispute_deadline_secs` must have passed since last state change
- Disputant must deposit `dispute_fee_escrow` lamports
- In re-dispute, only the counterparty can dispute

---

### 7. `resolve_dispute`

Resolves a dispute by sending tokens to the winner (authority only).

```rust
pub fn resolve_dispute(
    ctx: Context<ResolveDispute>,
    escrow_id: u64,
) -> Result<()>
```

**Process:**

1. Verifies the caller is the authority
2. Transfers tokens to specified winner
3. Calculates and distributes dispute funds
4. Updates available fees
5. Closes escrow account
6. Emits `DisputeResolved` event

**Requirements:**

- Only authority can resolve disputes
- Escrow must be in `Dispute` or `ReDispute` state
- Must specify the winner (`to` in accounts)

---

### 8. `withdraw_spl`

Withdraws accumulated fees from a specific token (authority only).

```rust
pub fn withdraw_spl(
    ctx: Context<WithdrawSpl>,
) -> Result<()>
```

**Process:**

1. Verifies the caller is the authority
2. Transfers available tokens from vault to authority
3. Updates available counter in vault

**Requirements:**

- Only authority can withdraw
- Must have available funds in the vault of the specified mint

---

### 9. `update_global_config`

Updates the program's global configuration parameters (authority only).

```rust
pub fn update_global_config(
    ctx: Context<UpdateGlobalConfig>,
    authority: Option<Pubkey>,
    fee_bps: Option<u16>,
    fiat_deadline_secs: Option<i64>,
    dispute_deadline_secs: Option<i64>,
    dispute_fee_escrow: Option<u64>,
) -> Result<()>
```

**Requirements:**

- Only the current authority can call this function
- All parameters are optional - provide only what needs to be changed
- Changes affect all future escrows, not existing ones

---

## 📊 States and Accounts

### EscrowState

```rust
pub enum EscrowState {
    Open(i64),       // Escrow created, waiting for payment (timestamp)
    FiatPaid(i64),   // Buyer marked as paid (timestamp)
    Dispute(i64),    // In dispute (timestamp)
    ReDispute(i64),  // In re-dispute (timestamp)
}
```

### EscrowDisputedBy

```rust
pub enum EscrowDisputedBy {
    Nobody,   // No disputes
    Seller,   // Disputed by seller
    Buyer,    // Disputed by buyer
}
```

## 📡 Events

The program emits the following events for tracking:

### EscrowCreated

```rust
pub struct EscrowCreated {
    pub id: u64,
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}
```

### MarkEscrowAsPaid

```rust
pub struct MarkEscrowAsPaid {
    pub id: u64,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub marked_at: i64,
}
```

### TokensReleased

```rust
pub struct TokensReleased {
    pub id: u64,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}
```

### EscrowCancelled

```rust
pub struct EscrowCancelled {
    pub id: u64,
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub returned_amount: u64,
    pub canceled_at: i64,
}
```

### DisputeCreated

```rust
pub struct DisputeCreated {
    pub id: u64,
    pub disputant: Pubkey,
    pub disputed_at: i64,
}
```

### DisputeResolved

```rust
pub struct DisputeResolved {
    pub id: u64,
    pub winner: Pubkey,
    pub resolved_at: i64,
}
```

## 🧪 Tests

The project includes a complete TypeScript test suite that covers all program flows:

### Included Tests

1. **`initialize`**: Global configuration initialization
2. **`create_escrow`**: Escrow creation with tokens
3. **`mark_escrow_as_paid`**: Mark as paid by buyer
4. **`release_tokens_in_escrow`**: Token release with signature
5. **`cancel_escrow`**: Escrow cancellation by timeout
6. **`create_dispute`**: Dispute and re-dispute creation
7. **`resolve_dispute`**: Dispute resolution by authority
8. **`withdraw_spl`**: Accumulated fees withdrawal
9. **`update_global_config`**: Update global configuration parameters

### Test Structure

```typescript
tests/
├── p2p.test.ts           # Main test suite
└── utils/
    ├── accounts.ts       # Helpers to fetch accounts
    ├── constants.ts      # Testing constants
    ├── events.ts         # Event listeners
    ├── functions.ts      # Auxiliary functions
    └── parsers.ts        # Data parsers
```

### Running Tests

```bash
# Run all tests
anchor test

# Or with yarn
yarn test

# Only compile without tests
anchor build
```

### Test Constants

```typescript
export const FEE_BPS = 100; // 1% fee
export const FIAT_DEADLINE_SECS = new BN(1800); // 30 minutes
export const DISPUTE_DEADLINE_SECS = new BN(43200); // 12 hours
export const DISPUTE_FEE_ESCROW = new BN(1_000_000); // 0.001 SOL
```

## 🚀 Installation and Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70+
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) 1.18+
- [Anchor Framework](https://www.anchor-lang.com/docs/installation) 0.31+
- [Node.js](https://nodejs.org/) 18+
- [Yarn](https://yarnpkg.com/)

### Installation

1. **Clone the repository:**

   ```bash
   git clone <repository-url>
   cd p2p
   ```

2. **Install Node dependencies:**

   ```bash
   yarn install
   ```

3. **Install Rust dependencies:**

   ```bash
   cargo build-sbf
   ```

4. **Configure Solana CLI:**

   ```bash
   # Configure for local development
   solana config set --url localhost

   # Generate keypair if it doesn't exist
   solana-keygen new
   ```

## 💻 Local Development

### 1. Start Local Validator

In a terminal, start the Solana local validator:

```bash
solana-test-validator
```

This will start a validator at `http://localhost:8899`.

### 2. Build the Program

In another terminal, compile the program:

```bash
anchor build
```

This will generate:

- Program binary in `target/deploy/`
- IDL in `target/idl/p2p.json`
- TypeScript types in `target/types/p2p.ts`

### 3. Local Deploy

Deploy the program to the local validator:

```bash
anchor deploy
```

Or use the pre-configured program ID:

```bash
# The program is configured with this ID
# GQKqoMVW3BuSzFRRkfeVsLPArAkRiZkd1vkVNGeqRmJG
```

### 4. Run Tests

Run the complete test suite:

```bash
anchor test
```

Or only tests without rebuild:

```bash
yarn test
```

### 5. Useful Commands

```bash
# View program logs
solana logs

# View account information
solana account <PUBKEY>

# View balance
solana balance

# Airdrop SOL for testing
solana airdrop 2

# Clean and rebuild
anchor clean && anchor build
```

## 🌐 Deployment

### Devnet

1. **Configure Solana for Devnet:**

   ```bash
   solana config set --url devnet
   ```

2. **Request SOL from Devnet:**

   ```bash
   solana airdrop 2
   ```

3. **Deploy to Devnet:**

   ```bash
   anchor build
   anchor deploy --provider.cluster devnet
   ```

4. **Update Anchor.toml:**
   ```toml
   [provider]
   cluster = "devnet"
   ```

### Mainnet

⚠️ **WARNING**: Before deploying to mainnet, make sure to:

- Have thoroughly tested on devnet
- Have a professional security audit
- Configure the authority correctly
- Have sufficient SOL for deployment

```bash
solana config set --url mainnet-beta
anchor build --verifiable
anchor deploy --provider.cluster mainnet
```

## 📝 Program Configuration

### Recommended Parameters

For production, consider these values:

```rust
fee_bps: 100,                    // 1% fee
fiat_deadline_secs: 1800,        // 30 minutes to pay
dispute_deadline_secs: 43200,    // 12 hours before being able to dispute
dispute_fee_escrow: 1_000_000,   // 0.001 SOL (~$0.10 at current price)
```

### Authority

The authority is the account that can:

- Resolve disputes
- Withdraw accumulated fees
- Update global configuration parameters (fees, deadlines, dispute deposit)
- Transfer authority to a new account

Make sure to use a secure wallet (e.g., Ledger) for mainnet.

## 🛡️ Security

### Considerations

1. **Signature Validation**: All token releases require seller's signature
2. **Timeouts**: Escrows have deadlines to avoid permanently locked funds
3. **Disputes**: Two-level system to resolve conflicts
4. **Limited Authority**: Can only resolve disputes, not move funds arbitrarily
5. **Dispute Deposits**: Protection against dispute spam

### Recommendations

- Always test on devnet first
- Use small amounts initially on mainnet
- Keep the authority in cold storage
- Monitor events to detect suspicious activity
- Implement a frontend with additional validations

## 📄 License

ISC

---

## 🤝 Contributing

Contributions are welcome. Please:

1. Fork the project
2. Create a branch for your feature (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📞 Support

For questions or support, please open an issue in the repository.

---

**Program ID**: `GQKqoMVW3BuSzFRRkfeVsLPArAkRiZkd1vkVNGeqRmJG`

**Cluster**: Localnet (configured in `Anchor.toml`)
