# Vero Core Contracts

The decentralized brain of the Vero Oracle Network. Built on **Soroban** (Stellar's smart contract platform), these contracts replace single-maintainer bias with **Guardian consensus** — a threshold-based voting system that verifies GitHub contributions on-chain before any reward is released.

---

## Architecture

```
GitHub Webhook
      │
      ▼
  Vero Relayer  (off-chain service)
      │  register_task(github_id)
      ▼
┌─────────────────────────────────┐
│        VeroOracle Contract      │
│                                 │
│  DataKey::Admin                 │
│  DataKey::Guardian(Address)     │
│  DataKey::Task(u64)             │
│                                 │
│  init()  ──► sets Admin         │
│  add_guardian() ──► whitelist   │
│  register_task() ──► Task{}     │
│  vote() ──► consensus check     │
│    └─► votes_for >= threshold   │
│          emit "verified" event  │
└─────────────────────────────────┘
      │  "verified" event
      ▼
  Reward Distributor (future contract)
```

**Key roles:**
- **Admin** — deploys and manages the Guardian whitelist.
- **Guardian** — trusted developer who casts on-chain votes for verified PRs.
- **Relayer** — off-chain service that watches GitHub webhooks and calls `register_task`.

---

## Contract: VeroOracle

### Data Types

```rust
#[contracttype]
pub struct Task {
    pub github_id: u64,   // GitHub PR / issue number
    pub votes_for: u32,   // accumulated Guardian votes
    pub resolved: bool,   // true once threshold is reached
}

#[contracttype]
pub enum DataKey {
    Admin,
    Guardian(Address),
    Task(u64),
}
```

### Public Interface

| Function | Auth | Description |
|---|---|---|
| `init(admin)` | — | One-time setup, stores Admin address |
| `add_guardian(guardian)` | Admin | Whitelists a Guardian address |
| `register_task(github_id)` | Relayer/Admin | Creates a pending Task |
| `vote(guardian, github_id)` | Guardian | Casts a vote; emits `verified` at threshold |

### Voting & Consensus

```rust
pub fn vote(env: Env, guardian: Address, github_id: u64) {
    guardian.require_auth();
    assert!(env.storage().instance().has(&DataKey::Guardian(guardian.clone())));

    let mut task: Task = env.storage().instance()
        .get(&DataKey::Task(github_id)).unwrap();
    assert!(!task.resolved, "Already resolved");

    task.votes_for += 1;
    if task.votes_for >= 3 {
        task.resolved = true;
        env.events().publish(
            (Symbol::new(&env, "verified"), github_id),
            github_id,
        );
    }
    env.storage().instance().set(&DataKey::Task(github_id), &task);
}
```

The threshold is **3 Guardian votes**. Once reached, a `verified` event is emitted on-chain and the task is locked — no further votes are accepted.

---

## Quick Start

**Prerequisites:** Rust `wasm32-unknown-unknown` target, Soroban CLI.

```bash
# 1. Build
cargo build --target wasm32-unknown-unknown --release

# 2. Deploy to Testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/vero_core_contracts.wasm \
  --network testnet

# 3. Init
soroban contract invoke --id <CONTRACT_ID> \
  --fn init -- --admin <ADMIN_ADDRESS>

# 4. Add a Guardian
soroban contract invoke --id <CONTRACT_ID> \
  --fn add_guardian -- --guardian <GUARDIAN_ADDRESS>

# 5. Register a task (Relayer)
soroban contract invoke --id <CONTRACT_ID> \
  --fn register_task -- --github_id 42

# 6. Vote
soroban contract invoke --id <CONTRACT_ID> \
  --fn vote -- --guardian <GUARDIAN_ADDRESS> --github_id 42
```

---

## Project Structure

```
vero-core-contracts/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Contract entry point
│   ├── types.rs        # Shared contracttypes
│   ├── guardian.rs     # Guardian management
│   ├── task.rs         # Task registration & voting
│   └── events.rs       # Event helpers
└── tests/
    └── integration.rs  # Soroban test harness
```

---

## License

MIT
