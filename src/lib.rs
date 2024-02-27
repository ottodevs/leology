//! Leology is an Aleo testing framework that makes developing Leo programs faster, easier, and safer. It includes the most commonly used RPC functions and can be run deterministically to make development a breeze.
//!
//! - Easy setup of Local Testnet
//! - Generate Accounts for testing
//! - Programmatic use in Rust to run tests
//! - Testing of both private and public values
//! - Local deployment of programs
//! - Aleo RPC support
//!

pub mod api;
pub mod bindings;
pub mod account;
pub use api::new_account;
pub use account::Account;
pub use snarkvm::circuit::AleoV0 as Aleo;
pub use snarkvm::package::Package;
pub use snarkvm::ledger::block::Transaction;
pub use snarkvm::prelude::*;

pub use snarkvm::console::network::MainnetV0 as Nw;
pub const SNARKVM_CONTRACTS_BUILD_FOLDER: &str = "build";
pub const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:3030";
