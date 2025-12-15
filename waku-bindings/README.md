# Waku Rust bindings

[<img alt="github" src="https://img.shields.io/badge/github-Github-red?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/logos-messaging/logos-messaging-rust-bindings)
[<img alt="crates.io" src="https://img.shields.io/crates/v/waku-bindings.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/waku-bindings)
[<img alt="docs.rs" src="https://img.shields.io/badge/doc/waku-bindings-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/waku-bindings)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/logos-messaging/logos-messaging-rust-bindings/main.yml?branch=master" height="20">](https://github.com/logos-messaging/logos-messaging-rust-bindings/actions/workflows/main.yml?query=branch%3Amaster)

Rust api on top of [`waku-sys`](https://crates.io/crates/waku-sys) bindgen bindings to  [c ffi bindings](https://github.com/status-im/go-waku/blob/v0.2.2/library/README.md).


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
waku-bindings = "0.1.0"
```


## About [Waku](https://waku.org/)

Waku is the communication layer for Web3. Decentralized communication that scales.

Private. Secure. Runs anywhere.

### What is Waku?

Waku is a suite of privacy-preserving, peer-to-peer messaging protocols.

Waku removes centralized third parties from messaging, enabling private, secure, censorship-free communication with no single point of failure.

Waku provides privacy-preserving capabilities, such as sender anonymity,metadata protection and unlinkability to personally identifiable information.

Waku is designed for generalized messaging, enabling human-to-human, machine-to-machine or hybrid communication.

Waku runs everywhere: desktop, server, including resource-restricted devices, such as mobile devices and browsers.
How does it work?

The first version of Waku had its origins in the Whisper protocol, with optimizations for scalability and usability. Waku v2 is a complete rewrite. Its relay protocol implements pub/sub over libp2p, and also introduces additional capabilities:

1. Retrieving historical messages for mostly-offline devices.
2. Adaptive nodes, allowing for heterogeneous nodes to contribute.
3. Bandwidth preservation for light nodes.

This makes it ideal for running a p2p protocol on mobile, or in other similarly resource-restricted environments.



Read the [Waku docs](https://docs.wakuconnect.dev/)