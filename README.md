# habit-tracker
📖 User Story
As a person trying to build consistent daily habits,
I want a decentralized habit tracker that counts my daily check-ins on the blockchain,
So that I have an immutable, verifiable record of my consistency that I truly own and can prove to others or use as collateral for commitment contracts.

Key Features

Initialize personal habit counters on-chain
Increment counter once per day when checking in
Prevent gaming the system with cooldown periods
View total streak count and last check-in time
Immutable proof of consistency stored on Solana




### Description

The Habit Tracker is a decentralized application (dApp) built on Solana using the Anchor framework. Its core purpose is to provide users with a verifiable, immutable record of their commitment to daily habits. Every successful check-in results in a signed transaction that permanently records the streak count and the timestamp of the last activity. The program enforces a strict 24-hour cooldown period between check-ins to ensure genuine daily consistency.

### Key Features

- **Immutable Streak Tracking**: All habit accounts and check-in history are stored on-chain in secure PDAs.

- **Enforced Cooldown**: The program uses the Solana clock to enforce a strict 24-hour (86,400 seconds) cooldown, preventing immediate duplicate check-ins.

- **Unique Habit Creation**: Users can create multiple distinct habits, each tied to a unique Program Derived Address (PDA).

- **Streak Badges**: Tracks and records 7-day and 30-day streak achievements directly on the Habit account state.

## Program Architecture

The program leverages PDAs for state security and focuses on two primary instructions to manage the habit lifecycle.

### PDA Usage

The program uses Program Derived Addresses (PDAs) for the Habit account. This is essential for:

Allowing the program to manage the account's life cycle.

Allowing a single owner to create multiple, distinct, deterministic habit accounts.

### Program Instructions

**Instructions Implemented:**

- **create_habit(name: String, habit_nonce: u8)**: Initializes a new Habit PDA account, setting the owner, name, and initial values (count = 0, last_check_in = 0).

- **Validation**: Enforces NameTooLong constraint (max 40 bytes).

- **check_in(habit_nonce: u8)**: Increments the count field and updates last_check_in to the current Solana clock timestamp.

- **Validation**: Enforces the 24-hour cooldown by throwing CooldownNotElapsed if the required time has not passed since last_check_in. Also verifies the signer is the account owner via has_one.

### Account Structure

```rust
#[account]
pub struct Habit {
    pub owner: Pubkey,        // The public key of the user who owns the habit.
    pub name: String,         // The descriptive name of the habit (Max 40 bytes).
    pub count: u64,           // The current consecutive check-in streak length.
    pub last_check_in: i64,   // Unix timestamp (seconds) of the last successful check-in.
    pub created_at: i64,      // Unix timestamp when habit was created.
    pub bump: u8,             // The bump seed for PDA verification.
    pub streak7: bool,        // Flag: True if count >= 7.
    pub streak30: bool,       // Flag: True if count >= 30.
}
