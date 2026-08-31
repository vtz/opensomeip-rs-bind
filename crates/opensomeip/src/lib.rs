//! Safe Rust bindings for OpenSOME/IP.
//!
//! This crate wraps the opensomeip C API (`opensomeip-sys`) in safe,
//! idiomatic Rust types compatible with the Ferrocene subset.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unsafe_op_in_unsafe_fn)]

// Safe API will be added in OSI-RS-4
