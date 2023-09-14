# near-sdk-contract-tools

> Helpful functions and macros for developing smart contracts on NEAR Protocol.

This package is a collection of common tools and patterns in NEAR smart contract development:

- Storage fee management
- Owner pattern + derive macro
- Pause pattern + derive macro
- Role-based access control
- Derive macros for NEP standards:
  - [NEP-297](https://nomicon.io/Standards/EventsFormat) (events)
  - [NEP-141](https://nomicon.io/Standards/Tokens/FungibleToken/Core) (fungible token), extension [NEP-148](https://nomicon.io/Standards/Tokens/FungibleToken/Metadata)
  - [NEP-171](https://nomicon.io/Standards/NonFungibleToken/NonFungibleToken) (non-fungible token), extensions [NEP-177](https://nomicon.io/Standards/Tokens/NonFungibleToken/Metadata), [NEP-178](https://nomicon.io/Standards/Tokens/NonFungibleToken/ApprovalManagement), [NEP-181](https://nomicon.io/Standards/Tokens/NonFungibleToken/Enumeration)

Not to be confused with [`near-contract-standards`](https://crates.io/crates/near-contract-standards), which contains official implementations of standardized NEPs. This crate is intended to be a complement to `near-contract-standards`.

You can think of this collection of common tools and patterns (mostly in the form of [derive macros](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros)) as a sort of [OpenZeppelin](https://docs.openzeppelin.com/contracts/4.x/) for NEAR.

**WARNING:** This is still early software, and there may be breaking changes between versions. I'll try my best to keep the docs & changelogs up-to-date. Don't hesitate to create an issue if find anything wrong.

## Installation

```bash
cargo add near-sdk-contract-tools
```

## Examples

See also: [the full integration tests](tests/macros/mod.rs).

### Owner

```rust
use near_sdk::{near_bindgen, AccountId};
use near_sdk_contract_tools::{owner::Owner, Owner};

#[derive(Owner)]
#[near_bindgen]
struct Contract {
    // ...
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let mut contract = Self {
            // ...
        };

        Owner::init(&mut contract, &owner_id);

        contract
    }

    pub fn owner_only(&self) {
        Self::require_owner();

        // ...
    }
}
```

The `Owner` derive macro exposes the following methods to the blockchain:

```rust, ignore
fn own_get_owner(&self) -> Option<AccountId>;
fn own_get_proposed_owner(&self) -> Option<AccountId>;
fn own_renounce_owner(&mut self);
fn own_propose_owner(&mut self, account_id: Option<AccountId>);
fn own_accept_owner(&mut self);
```

### Events

The `#[event]` macro can be applied to structs or enums.

```rust
use near_sdk_contract_tools::{event, standard::nep297::Event};

#[event(standard = "nft", version = "1.0.0")]
pub struct MintEvent {
    pub owner_id: String,
    pub token_id: String,
}

let e = MintEvent {
    owner_id: "account".to_string(),
    token_id: "token_1".to_string(),
};

// Emits the event to the blockchain
e.emit();
```

### Fungible Token

To create a contract that is compatible with the NEP-141 and NEP-148 standards, that emits standard-compliant (NEP-141, NEP-297) events.

```rust
use near_sdk_contract_tools::FungibleToken;
use near_sdk::near_bindgen;

#[derive(FungibleToken)]
#[fungible_token(
    name = "My Fungible Token",
    symbol = "MYFT",
    decimals = 18,
    no_hooks
)]
#[near_bindgen]
struct FungibleToken {
    // ...
}
```

Standalone macros for each individual standard also exist.

### Non-fungible Token

Use the `NonFungibleToken` derive macro to implement NEP-171, NEP-177, NEP-178, and NEP-181, with NEP-297 events.

```rust
use near_sdk::{
    borsh::{self, BorshSerialize, BorshDeserialize},
    PanicOnDefault,
    near_bindgen,
};
use near_sdk_contract_tools::nft::*;

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault, NonFungibleToken)]
#[non_fungible_token(no_core_hooks, no_approval_hooks)]
#[near_bindgen]
pub struct MyNft {}
```

### Macro Combinations

One may wish to combine the features of multiple macros in one contract. All of the macros are written such that they will work in a standalone manner, so this should largely work without issue. However, sometimes it may be desirable for the macros to work in _combination_ with each other. For example, to make a fungible token pausable, use the fungible token hooks to require that a contract be unpaused before making a token transfer:

```rust
use near_sdk_contract_tools::{
    pause::Pause,
    standard::nep141::{Nep141Hook, Nep141Transfer},
    FungibleToken, Pause,
};
use near_sdk::near_bindgen;

#[derive(FungibleToken, Pause)]
#[fungible_token(name = "Pausable Fungible Token", symbol = "PFT", decimals = 18)]
#[near_bindgen]
struct Contract {}

impl Nep141Hook for Contract {
    fn before_transfer(&mut self, _transfer: &Nep141Transfer) {
        Contract::require_unpaused();
    }
}
```

Note: Hooks can be disabled using `#[nep141(no_hooks)]` or `#[fungible_token(no_hooks)]`.

### Custom Crates

If you are a library developer, have modified a crate that one of the `near-sdk-contract-tools` macros uses (like `serde` or `near-sdk`), or are otherwise using a crate under a different name, you can specify crate names in macros like so:

```rust, ignore
#[event(
    // ...
    crate = "near_sdk_contract_tools",
    macros = "near_sdk_contract_tools_macros",
    serde = "serde",
)]
// ...

#[derive(Owner)]
#[owner(
    // ...
    near_sdk = "near_sdk",
)]
```

## Other Tips

### [Internal vs. External Methods](https://youtu.be/kJzes_UP5j0?t=2172)

Internal methods are not available to be callable via the blockchain. External ones are public and can be called by other contracts.

### [Pull pattern](https://youtu.be/kJzes_UP5j0?t=2213)

Proposing ownership (rather than transferring directly) is a generally good practice because it prevents you from accidentally transferring ownership to an account that nobody has access to, bricking the contract.

### [Expand](https://youtu.be/kJzes_UP5j0?t=1790)

[`cargo expand`](https://crates.io/crates/cargo-expand) will generate one huge Rust file with all of the macros have been processed:

```text
cargo install cargo-expand
cargo expand > expanded.rs
```

### [Slots](https://youtu.be/kJzes_UP5j0?t=2527)

See [`src/slot.rs`](src/slot.rs). Slots are thin wrappers over a storage key.

### [`assert_one_yocto()`](https://youtu.be/kJzes_UP5j0?t=2989)

`near_sdk::assert_one_yocto()` is a function that requires a full access key (by requiring a deposit of one yoctonear, the smallest possible unit of NEAR).

If a user connects their NEAR account to a dapp, the dapp will still not be able to call functions that call `assert_one_yocto()`, since function call access keys are not allowed to transfer native tokens. These function will require a signature from a full access key, usually involving a confirmation screen in the user's wallet.

## Contributing

### Setup

Run `git config core.hooksPath hooks/` to set up commit hooks.

### Build and test

Install `cargo-make` if it is not installed already:

```text
cargo install cargo-make
```

Run tests:

```text
cargo test
cd workspaces-tests
cargo make test
```

## Audit

This library has been [audited](./documents/NEAR%20Contract%20Tools%20-%20Final%20-%2005.05.2023.pdf) by [Kudelski Security](https://www.kudelskisecurity.com/).

## Authors

- Jacob Lindahl [@sudo_build](https://twitter.com/sudo_build)

---

(Formerly known as [`near-contract-tools`](https://crates.io/crates/near-contract-tools).)
