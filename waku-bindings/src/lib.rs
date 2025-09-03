//! # Waku
//!
//! Implementation on top of [`waku-bindings`](https://rfc.vac.dev/spec/36/)
pub mod general;
mod macros;
pub mod node;

// Re-export the LibwakuResponse type to make it accessible outside this module
pub use general::libwaku_response::LibwakuResponse;

// Required so functions inside libwaku can call RLN functions even if we
// use it within the bindings functions
#[allow(clippy::single_component_path_imports)]
#[allow(unused)]
use rln;

pub use node::{
    Initialized, Key, Multiaddr, PublicKey, RLNConfig, Running, SecretKey, WakuEvent,
    WakuMessageEvent, WakuNodeConfig, WakuNodeHandle,
};

pub use general::contenttopic::{Encoding, WakuContentTopic};
pub use general::{messagehash::MessageHash, Result, WakuMessage, WakuMessageVersion};
