// Copyright (c) 2025 Vinicius Tadeu Zein
// SPDX-License-Identifier: Apache-2.0

//! Safe Rust bindings for OpenSOME/IP.
//!
//! This crate wraps the opensomeip C API (`opensomeip-sys`) in safe,
//! idiomatic Rust types compatible with the Ferrocene subset.
//!
//! # Feature Flags
//!
//! - **`std`** (default): Enables `Vec`-returning methods, async RPC
//!   callbacks with closures, and `std::error::Error` implementation.
//!   Disable for `no_std` environments.
//!
//! @implements REQ_RUST_004, REQ_RUST_005, REQ_RUST_007

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unsafe_op_in_unsafe_fn)]

pub mod e2e;
pub mod error;
pub mod events;
pub mod message;
pub mod rpc;
pub mod sd;
pub mod serialization;
pub mod tp;
pub mod transport;
pub mod types;

// Re-export primary types at crate root for convenience.
pub use e2e::E2EProtection;
pub use error::{Result, SomeIpError};
pub use events::{EventPublisher, EventSubscriber};
pub use message::{SomeIpMessage, SomeIpMessageBuilder};
pub use rpc::{RpcClient, RpcServer};
pub use sd::{SdClient, SdServer};
pub use serialization::{Deserializer, Serializer};
pub use tp::TpManager;
pub use transport::{TcpTransport, UdpTransport};
pub use types::{Endpoint, MessageType, ReturnCode, TransportProtocol};

/// Returns the C API version as a packed `u32`.
///
/// @implements REQ_RUST_007
pub fn capi_version() -> u32 {
    // SAFETY: This is a simple query function with no side effects.
    unsafe { opensomeip_sys::opensomeip_capi_version() }
}

/// Returns the C API version as `(major, minor, patch)`.
pub fn capi_version_tuple() -> (u32, u32, u32) {
    (
        opensomeip_sys::OPENSOMEIP_CAPI_VERSION_MAJOR,
        opensomeip_sys::OPENSOMEIP_CAPI_VERSION_MINOR,
        opensomeip_sys::OPENSOMEIP_CAPI_VERSION_PATCH,
    )
}
