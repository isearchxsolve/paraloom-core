// `#[async_trait]` rewrites every `async fn` in an annotated trait into a
// `Pin<Box<dyn Future>>` return type and stamps `#[must_use]` on it. Clippy's
// `double_must_use` then fires on the generated attribute because a boxed future
// is already `#[must_use]`. That is a false positive in macro-generated code we
// cannot correct without forking the attribute, so allow it crate-wide rather
// than litter every `#[async_trait]` trait with an identical allow.
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
