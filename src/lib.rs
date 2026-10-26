// `#[async_trait]` expands every `async fn` into a wrapper that returns a
// `Pin<Box<dyn Future>>` (already `#[must_use]`) and re-applies the trait
// method's `#[must_use]`, which newer clippy releases flag as
// `double_must_use`. The attribute is emitted by the macro, not written by
// hand, so it cannot be removed at the source; silence the false positive for
// the whole crate.
#![allow(clippy::double_must_use)]

// Bridge module for cross-chain interactions
#[cfg(feature = "solana-bridge")]
pub mod bridge;
pub mod ceremony;
pub mod compute;
pub mod config;
pub mod consensus;
pub mod coordinator;
pub mod health;
pub mod network;
pub mod node;
pub mod privacy;
pub mod relayer;
pub mod resource;
pub mod storage;
pub mod task;
pub mod types;
pub mod utils;
pub mod validator;
pub mod web;

pub use config::Settings;
